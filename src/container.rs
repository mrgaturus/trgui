//! A list of widgets for dispatch Widget trait functions to specific widgets
//!
//! A Container implements Widget trait, so Containers can be nested. The widget list
//! of a Container cannot be modified after moved to a "parent" Container

use crate::group::{Group, GroupID};
use crate::state::{KeyState, MouseState};
use crate::widget::flags::*;
use crate::widget::{Boundaries, Dimensions, Flags, Widget, WidgetInternal};
use crate::{Decorator, Layout};

use std::ops::{Add, Sub};

const HANDLERS: Flags = FOCUS | GRAB | HOVER;
const REACTIVE: Flags = DRAW | UPDATE | LAYOUT | PREV_LAYOUT;
const FOCUSABLE: Flags = FOCUS | ENABLED | VISIBLE;

const PARTIAL: Flags = 0b000001_0000000000;

type WidgetList<T> = Vec<Box<dyn Widget<T>>>;
type InternalList<T> = Vec<WidgetInternal<T>>;

/// Widget List that handle widget trait functions
pub struct Container<T, DE: Decorator<T>> {
    widgets_i: InternalList<T>,
    widgets: WidgetList<T>,
    decorator: DE,
    layout: Box<dyn Layout<T>>,
    focus_id: Option<usize>,
    grab_id: Option<usize>,
    hover_id: Option<usize>,
}

impl<T: Sized + Copy + Clone, DE> Container<T, DE>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Default,
    DE: Decorator<T>,
{
    /// Creates a new Container with a Decorator and Layout
    pub fn new(decorator: DE, layout: Box<dyn Layout<T>>) -> Self {
        Container {
            widgets_i: InternalList::new(),
            widgets: WidgetList::new(),
            focus_id: Option::None,
            grab_id: Option::None,
            hover_id: Option::None,
            decorator,
            layout,
        }
    }

    /// Applies shrink_to_fit to widget list
    pub fn pack(mut self) -> Self {
        self.widgets_i.shrink_to_fit();
        self.widgets.shrink_to_fit();

        self
    }

    /// Adds a new widget to the list
    pub fn add_widget(&mut self, widget: Box<dyn Widget<T>>, flags: Flags, group: Group) {
        let mut internal = WidgetInternal::new(flags, group);
        internal.off(HANDLERS);
        internal.set_min_dimensions(widget.min_dimensions());

        self.widgets_i.push(internal);
        self.widgets.push(widget);
    }

    /// Adds a new widget to the list with initial bounds
    ///
    /// Useful when initial boundaries are required by the Layout
    pub fn add_widget_b(
        &mut self,
        widget: Box<dyn Widget<T>>,
        flags: Flags,
        group: Group,
        bounds: Boundaries<T>,
    ) {
        let mut internal =
            WidgetInternal::new_with((bounds.0, bounds.1), (bounds.2, bounds.3), flags, group);
        internal.off(HANDLERS);
        internal.set_min_dimensions(widget.min_dimensions());

        self.widgets_i.push(internal);
        self.widgets.push(widget);
    }

    fn step(&mut self, back: bool) {
        match self.focus_id {
            Some(mut id) => {
                let len = self.widgets.len() - 1;

                if back {
                    if id == 0 {
                        self.focus_id = None;
                        return;
                    }
                    id -= 1;
                } else {
                    if id == len {
                        self.focus_id = None;
                        return;
                    }
                    id += 1;
                }

                self.focus_id = Some(id);
            }
            None => {
                let mut val = 0;
                if back {
                    val = self.widgets.len() - 1;
                }

                self.focus_id = Some(val);
            }
        }
    }
}

impl<T: Sized + Copy + Clone, DE> Widget<T> for Container<T, DE>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Default,
    DE: Decorator<T>,
{
    /// Draw widgets from the list that have DRAW flag turned on
    ///
    /// This function is lazy, if none widget is found, the DRAW flag
    /// of the container turns off
    fn draw(&mut self, internal: &WidgetInternal<T>) -> bool {
        if internal.check(VISIBLE) {
            self.decorator.before(internal);

            let count = self
                .widgets_i
                .iter_mut()
                .zip(self.widgets.iter_mut())
                .filter(|(w_internal, _)| w_internal.check(DRAW))
                .fold(0, |_, (w_internal, widget)| {
                    let draw = widget.draw(w_internal);
                    if !draw {
                        w_internal.off(DRAW);
                    }

                    draw as usize
                });

            self.decorator.after(internal);

            count > 0
        } else {
            false
        }
    }

    /// Update widgets from the list that have UPDATE flag turned on
    ///
    /// This function is lazy, if none widget is found, the UPDATE flag
    /// of the container turns off
    fn update(&mut self, internal: &mut WidgetInternal<T>) {
        let count: usize;

        count = self
            .widgets_i
            .iter_mut()
            .zip(self.widgets.iter_mut())
            .filter(|(w_internal, _)| w_internal.check(UPDATE))
            .fold(0, |_, (w_internal, widget)| {
                let backup = w_internal.flags();

                widget.update(w_internal);
                internal.on(w_internal.drain(DRAW | LAYOUT | PREV_LAYOUT, PREV_LAYOUT));

                w_internal.replace(HANDLERS, backup);
                w_internal.check(UPDATE) as usize
            });

        if let Some(id) = self.focus_id {
            if !self.widgets_i[id].check(FOCUSABLE) {
                self.focus_out(internal);
            }
        }

        if internal.check(PREV_LAYOUT) {
            internal.off_on(PREV_LAYOUT, LAYOUT | PARTIAL);
        }

        internal.set(UPDATE, count > 0);
    }

    /// Apply the Layout to the list, calculate the absolute position and update the Decorator
    fn layout(&mut self, internal: &mut WidgetInternal<T>, all: bool) {
        let do_layout = all || internal.check(PARTIAL);

        if do_layout {
            self.layout
                .layout(&mut self.widgets_i, &internal.dimensions());
        }

        self.decorator.update(internal);

        self.widgets_i
            .iter_mut()
            .zip(self.widgets.iter_mut())
            .filter(|(w_internal, _)| do_layout || w_internal.check(LAYOUT))
            .for_each(|(w_internal, widget)| {
                w_internal.calc_absolute(internal.absolute_pos());
                widget.layout(w_internal, all);

                w_internal.set(DRAW, w_internal.check(VISIBLE));
                internal.on(w_internal.drain(DRAW | UPDATE, LAYOUT | PREV_LAYOUT));
            });

        if let Some(id) = self.focus_id {
            if !self.widgets_i[id].check(FOCUSABLE) {
                self.focus_out(internal);
            }
        }

        internal.off(LAYOUT | PARTIAL);
    }

    /// Search widgets that are members of a Group id and call the function of these widgets
    ///
    /// A Nested Container should be member of the same Group id, otherwise, the function couldn't
    /// be called on the widget of the nested Container
    fn handle_signal(&mut self, internal: &mut WidgetInternal<T>, group: GroupID) {
        self.widgets_i
            .iter_mut()
            .zip(self.widgets.iter_mut())
            .filter(|(w_internal, _)| {
                w_internal.check(SIGNAL) && w_internal.group().check_id(group)
            })
            .for_each(|(w_internal, widget)| {
                let backup = w_internal.flags();

                widget.handle_signal(w_internal, group);
                internal.on(w_internal.drain(REACTIVE, PREV_LAYOUT));

                w_internal.replace(HANDLERS, backup);
            });

        if let Some(id) = self.focus_id {
            if !self.widgets_i[id].check(FOCUSABLE) {
                self.focus_out(internal);
            }
        }

        if internal.check(PREV_LAYOUT) {
            internal.off_on(PREV_LAYOUT, LAYOUT | PARTIAL);
        }
    }

    /// Search the widget that the mouse is pointing and call the function of the widget
    fn handle_mouse(&mut self, internal: &mut WidgetInternal<T>, mouse: &MouseState<T>) {
        if self.grab_id.is_some() || !internal.check(GRAB) {
            let widget_n = self
                .grab_id
                .or(self
                    .hover_id
                    .filter(|&n| self.widgets_i[n].on_area(mouse.absolute_pos())))
                .or_else(|| {
                    self.widgets_i
                        .iter()
                        .enumerate()
                        .find_map(|(n, w_internal)| {
                            if w_internal.on_area(mouse.absolute_pos()) {
                                Some(n)
                            } else {
                                None
                            }
                        })
                });

            if let Some(n) = widget_n {
                let w_internal = unsafe {
                    &mut *(self.widgets_i.get_unchecked_mut(n) as *mut WidgetInternal<T>)
                };

                if w_internal.check(GRAB) {
                    w_internal.set(HOVER, w_internal.on_area(mouse.absolute_pos()))
                } else {
                    if let Some(id) = self.hover_id {
                        if id != n {
                            self.hover_out(internal);
                        }
                    }
                    self.hover_id = Some(n);
                    w_internal.on(HOVER);
                }

                self.widgets[n].handle_mouse(w_internal, mouse);
                internal.on(w_internal.drain(REACTIVE, PREV_LAYOUT));

                self.grab_id = Some(n).filter(|_| {
                    let grab = w_internal.check(GRAB);

                    internal.set(GRAB, grab);
                    grab
                });

                let focus_check = w_internal.check(FOCUSABLE);

                if let Some(id) = self.focus_id {
                    if id != n {
                        if focus_check {
                            self.focus_out(internal);
                            self.focus_id = Some(n);
                        } else {
                            w_internal.off(FOCUS);
                        }
                    } else if !focus_check {
                        self.focus_out(internal);
                    }
                } else if focus_check {
                    self.focus_id = Some(n);
                    internal.on(FOCUS);
                } else {
                    w_internal.off(FOCUS);
                }
            } else {
                if self.hover_id.is_some() {
                    self.hover_out(internal);
                }

                internal.set(GRAB, mouse.clicked());
            }
        } else {
            internal.set(GRAB, mouse.clicked());
        }

        if internal.check(PREV_LAYOUT) {
            internal.off_on(PREV_LAYOUT, LAYOUT | PARTIAL);
        }
    }

    /// Call the function of the focused widget
    fn handle_keys(&mut self, internal: &mut WidgetInternal<T>, key: &KeyState) {
        if let Some(id) = self.focus_id {
            let w_internal = &mut self.widgets_i[id];

            self.widgets[id].handle_keys(w_internal, key);
            internal.on(w_internal.drain(REACTIVE, PREV_LAYOUT));

            if w_internal.check(GRAB) {
                self.grab_id = Some(id);
                internal.on(GRAB);
            }

            if !w_internal.check(FOCUSABLE) {
                self.focus_out(internal);
            }

            if internal.check(PREV_LAYOUT) {
                internal.off_on(PREV_LAYOUT, LAYOUT | PARTIAL);
            }
        }
    }

    /// Get minimum dimensions provided by the Layout
    fn min_dimensions(&self) -> Dimensions<T> {
        self.layout.minimum_size(&self.widgets_i)
    }

    /// Step the focus id to the next widget that returns true on the function
    fn step_focus(&mut self, internal: &mut WidgetInternal<T>, back: bool) -> bool {
        let step_check = {
            if !self.widgets.is_empty() {
                if let Some(id) = self.focus_id {
                    let w_internal = &mut self.widgets_i[id];
                    let widget = &mut self.widgets[id];

                    let focus = widget.step_focus(w_internal, back);
                    internal.on(w_internal.drain(REACTIVE, PREV_LAYOUT));

                    if focus {
                        return true;
                    } else {
                        widget.focus_out(w_internal);

                        internal.on(w_internal.val(REACTIVE));
                        w_internal.off(FOCUS | PREV_LAYOUT);
                    }
                }
                self.step(back);

                while let Some(id) = self.focus_id {
                    let w_internal = &mut self.widgets_i[id];
                    let focus = w_internal.check(ENABLED | VISIBLE)
                        && self.widgets[id].step_focus(w_internal, back);

                    internal.on(w_internal.drain(REACTIVE, PREV_LAYOUT));

                    if focus {
                        w_internal.on(FOCUS);
                        return focus;
                    } else {
                        self.step(back);
                    }
                }
            }

            false
        };

        if internal.check(PREV_LAYOUT) {
            internal.off_on(PREV_LAYOUT, LAYOUT | PARTIAL);
        }

        step_check
    }

    /// Clear the hover index and call the function of the widget
    fn hover_out(&mut self, internal: &mut WidgetInternal<T>) {
        if let Some(id) = self.hover_id {
            let w_internal = &mut self.widgets_i[id];

            self.widgets[id].hover_out(w_internal);
            internal.on(w_internal.drain(REACTIVE, HOVER | PREV_LAYOUT));

            self.hover_id = Option::None;

            if internal.check(PREV_LAYOUT) {
                internal.off_on(PREV_LAYOUT, LAYOUT | PARTIAL);
            }
        }
    }

    /// Clear the focus index and call the function of the widget
    fn focus_out(&mut self, internal: &mut WidgetInternal<T>) {
        if let Some(id) = self.focus_id {
            let w_internal = &mut self.widgets_i[id];

            self.widgets[id].focus_out(w_internal);
            internal.on(w_internal.drain(REACTIVE, FOCUS | PREV_LAYOUT));

            self.focus_id = Option::None;

            if internal.check(PREV_LAYOUT) {
                internal.off_on(PREV_LAYOUT, LAYOUT | PARTIAL);
            }
        }
    }
}