//! A list of widgets for dispatch Widget trait functions to specific widgets
//!
//! A Container implements Widget trait, so Containers can be nested. The widget list
//! of a Container cannot be modified after moved to a "parent" Container

use crate::group::{Group, GroupID};
use crate::state::{KeyState, MouseState};
use crate::widget::flags::*;
use crate::widget::{Boundaries, Dimensions, Widget, WidgetInternal};
use crate::{Decorator, Layout};

use std::ops::{Add, Sub};

const HANDLERS: Flags = 0b11100000; // FOCUS | GRAB | HOVER
const REACTIVE: Flags = 0b11_00000110; // DRAW | UPDATE | LAYOUT | PREV_LAYOUT
const FOCUSABLE: Flags = 0b10011000; // FOCUS | ENABLED | VISIBLE

const DRAIN_FOCUS: Flags = 0b10_10000000; // FOCUS | PREV_LAYOUT
const PARTIAL_TURN: Flags = 0b101_00000000; // PARTIAL_TURN

const PARTIAL: Flags = 0b1_0000000000;

type WidgetList<T, CTX> = Vec<Box<dyn Widget<T, CTX>>>;
type InternalList<T> = Vec<WidgetInternal<T>>;

/// Widget List that handle widget trait functions
pub struct Container<T, CTX: Sized, DE: Decorator<T, CTX>> {
    widgets_i: InternalList<T>,
    widgets: WidgetList<T, CTX>,
    layout: Box<dyn Layout<T>>,
    focus_id: Option<usize>,
    mouse_id: Option<usize>,
    decorator: DE,
}

impl<T: Sized + Copy + Clone, CTX: Sized, DE> Container<T, CTX, DE>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Default,
    DE: Decorator<T, CTX>,
{
    /// Creates a new Container with a Decorator and Layout
    pub fn new(decorator: DE, layout: Box<dyn Layout<T>>) -> Self {
        Container {
            widgets_i: InternalList::new(),
            widgets: WidgetList::new(),
            layout,
            focus_id: None,
            mouse_id: None,
            decorator,
        }
    }

    /// Applies shrink_to_fit to widget list
    pub fn pack(mut self) -> Box<Self> {
        self.widgets_i.shrink_to_fit();
        self.widgets.shrink_to_fit();

        Box::new(self)
    }

    /// Adds a new widget to the list, initial bounds are (0, 0, 0, 0)
    pub fn add_widget(&mut self, widget: Box<dyn Widget<T, CTX>>, flags: Flags, group: Group) {
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
        widget: Box<dyn Widget<T, CTX>>,
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

    fn focus_check(&mut self, internal: &mut WidgetInternal<T>) {
        if let Some(id) = self.focus_id {
            let w_internal = &mut self.widgets_i[id];

            if !w_internal.check(FOCUSABLE) {
                self.widgets[id].focus_out(w_internal);
                internal.on(w_internal.drain(REACTIVE, DRAIN_FOCUS));

                self.focus_id = None;
            }
        }
    }
}

impl<T: Sized + Copy + Clone, CTX, DE> Widget<T, CTX> for Container<T, CTX, DE>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Default,
    CTX: Sized,
    DE: Decorator<T, CTX>,
{
    /// Draw widgets from the list that have DRAW flag turned on
    ///
    /// This function is lazy, if none widget is found, the DRAW flag
    /// of the container turns off
    fn draw(&mut self, internal: &WidgetInternal<T>, ctx: &mut CTX) -> bool {
        self.decorator.before(internal, ctx);

        let count = self
            .widgets_i
            .iter_mut()
            .zip(self.widgets.iter_mut())
            // DRAW | VISIBLE
            .filter(|(w_internal, _)| w_internal.check(0b00001010))
            .fold(0, |_, (w_internal, widget)| {
                let draw = widget.draw(w_internal, ctx);
                if !draw {
                    w_internal.off(DRAW);
                }

                draw as usize
            });

        self.decorator.after(internal, ctx);

        count > 0
    }

    /// Update widgets from the list that have UPDATE flag turned on
    ///
    /// This function is lazy, if none widget is found, the UPDATE flag
    /// of the container turns off
    fn update(&mut self, internal: &mut WidgetInternal<T>) {
        let count = self
            .widgets_i
            .iter_mut()
            .zip(self.widgets.iter_mut())
            .filter(|(w_internal, _)| w_internal.check(UPDATE))
            .fold(0, |_, (w_internal, widget)| {
                let backup = w_internal.flags;

                // DRAW | LAYOUT | PREV_LAYOUT
                widget.update(w_internal);
                internal.on(w_internal.drain(0b11_00000010, PREV_LAYOUT));

                w_internal.replace(HANDLERS, backup);
                w_internal.check(UPDATE) as usize
            });

        self.focus_check(internal);

        if internal.check(PREV_LAYOUT) {
            internal.off_on(PREV_LAYOUT, PARTIAL_TURN);
        }

        internal.turn(UPDATE, count > 0);
    }

    /// Apply the Layout to the list, calculate the absolute position and update the Decorator
    fn layout(&mut self, internal: &mut WidgetInternal<T>, complete: bool) {
        let do_layout = complete || internal.check(PARTIAL);

        if do_layout {
            self.layout.layout(&mut self.widgets_i, internal);

            self.decorator.update(internal);
        }

        self.widgets_i
            .iter_mut()
            .zip(self.widgets.iter_mut())
            .filter(|(w_internal, _)| do_layout || w_internal.check(LAYOUT))
            .for_each(|(w_internal, widget)| {
                w_internal.set_pivot(internal.absolute_pos());

                let backup = w_internal.flags;
                widget.layout(w_internal, complete);

                // DRAW | UPDATE & LAYOUT | PREV_LAYOUT
                w_internal.turn(DRAW, w_internal.check(VISIBLE));
                internal.on(w_internal.drain(0b00000110, 0b11_00000000));

                w_internal.replace(HANDLERS, backup);
            });

        self.focus_check(internal);

        internal.off(PARTIAL_TURN);
    }

    /// Search widgets that are members of a Group id and call the function of these widgets
    ///
    /// A Nested Container should be member of the same Group id, otherwise, the function couldn't
    /// be called on the widget of the nested Container
    fn handle_signal(&mut self, internal: &mut WidgetInternal<T>, group: GroupID) {
        let mut n_focus: Option<usize> = None;

        self.widgets_i
            .iter_mut()
            .zip(self.widgets.iter_mut())
            .enumerate()
            .filter(|(_, (w_internal, _))| {
                w_internal.check(SIGNAL) && w_internal.group().check_id(group)
            })
            .for_each(|(n, (w_internal, widget))| {
                let backup = w_internal.flags;

                widget.handle_signal(w_internal, group);
                internal.on(w_internal.drain(REACTIVE, PREV_LAYOUT));

                // Check if focus flag is changed and check if the widget is focusable
                if (w_internal.flags ^ backup) & FOCUS == FOCUS && w_internal.check(FOCUSABLE) {
                    n_focus = Some(n);
                }

                w_internal.replace(HANDLERS, backup);
            });

        if let Some(n_id) = n_focus {
            if let Some(id) = self.focus_id.replace(n_id) {
                let o_internal = &mut self.widgets_i[id];

                self.widgets[id].focus_out(o_internal);
                internal.on(o_internal.drain(REACTIVE, DRAIN_FOCUS));
            }

            unsafe {
                self.widgets_i.get_unchecked_mut(n_id).on(FOCUS);
            }
        } else {
            self.focus_check(internal);
        }

        if internal.check(PREV_LAYOUT) {
            internal.off_on(PREV_LAYOUT, PARTIAL_TURN);
        }
    }

    /// Search the widget that the mouse is pointing and call the function of the widget
    fn handle_mouse(&mut self, internal: &mut WidgetInternal<T>, mouse: &MouseState<T>) {
        if self.mouse_id.is_some() || !internal.check(GRAB) {
            let widget_n = self
                .mouse_id
                .filter(|&n| {
                    internal.check(GRAB) || self.widgets_i[n].p_intersect(mouse.absolute_pos())
                })
                .or_else(|| {
                    self.widgets_i
                        .iter()
                        .enumerate()
                        .find_map(|(n, w_internal)| {
                            if w_internal.p_intersect(mouse.absolute_pos()) {
                                Some(n)
                            } else {
                                None
                            }
                        })
                });

            if widget_n != self.mouse_id {
                if let Some(id) = std::mem::replace(&mut self.mouse_id, widget_n) {
                    let w_internal = &mut self.widgets_i[id];

                    // HOVER | GRAB | PREV_LAYOUT
                    self.widgets[id].hover_out(w_internal);
                    internal.on(w_internal.drain(REACTIVE, 0b10_01100000));
                }
            }

            if let Some(n) = widget_n {
                let w_internal = unsafe { self.widgets_i.get_unchecked_mut(n) };

                if w_internal.check(GRAB) {
                    w_internal.turn(HOVER, w_internal.p_intersect(mouse.absolute_pos()));
                } else {
                    w_internal.on(HOVER);
                }

                self.widgets[n].handle_mouse(w_internal, mouse);
                internal.on(w_internal.drain(REACTIVE, PREV_LAYOUT));

                internal.turn(GRAB, w_internal.check(GRAB));

                // ENABLED | VISIBLE
                let focus_check = w_internal.flags & FOCUSABLE ^ 0b00011000;

                // Check if FOCUS is turned on and if ENABLED or VISIBLE is turned off
                if focus_check & FOCUS == FOCUS && focus_check > FOCUS {
                    self.widgets[n].focus_out(w_internal);
                    internal.on(w_internal.drain(REACTIVE, DRAIN_FOCUS));
                }

                if let Some(id) = self.focus_id {
                    if id != n {
                        if focus_check == FOCUS {
                            let o_internal = &mut self.widgets_i[id];

                            self.widgets[id].focus_out(o_internal);
                            internal.on(o_internal.drain(REACTIVE, DRAIN_FOCUS));

                            self.focus_id = widget_n;
                        }
                    } else if focus_check != FOCUS {
                        self.focus_id = None;
                    }
                } else if focus_check == FOCUS {
                    self.focus_id = widget_n;
                    internal.on(FOCUS);
                }
            } else {
                if mouse.m_count > 0 {
                    internal.on(GRAB);
                }
            }
        } else {
            if mouse.m_count == 0 {
                internal.off(GRAB);
            }
        }

        if internal.check(PREV_LAYOUT) {
            internal.off_on(PREV_LAYOUT, PARTIAL_TURN);
        }
    }

    /// Call the function of the focused widget
    fn handle_keys(&mut self, internal: &mut WidgetInternal<T>, key: KeyState) {
        if let Some(id) = self.focus_id {
            let w_internal = &mut self.widgets_i[id];
            let widget = &mut self.widgets[id];
            let backup = w_internal.flags;

            widget.handle_keys(w_internal, key);
            internal.on(w_internal.drain(REACTIVE, PREV_LAYOUT));

            // HOVER | GRAB
            w_internal.replace(0b01100000, backup);

            if !w_internal.check(FOCUSABLE) {
                widget.focus_out(w_internal);
                internal.on(w_internal.drain(REACTIVE, DRAIN_FOCUS));

                self.focus_id = None;
            }

            if internal.check(PREV_LAYOUT) {
                internal.off_on(PREV_LAYOUT, PARTIAL_TURN);
            }
        }
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
                        internal.on(w_internal.drain(REACTIVE, DRAIN_FOCUS));
                    }
                }
                self.step(back);

                while let Some(id) = self.focus_id {
                    let w_internal = &mut self.widgets_i[id];
                    let focus = w_internal.check(0b00011000) // ENABLED | VISIBLE
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
            internal.off_on(PREV_LAYOUT, PARTIAL_TURN);
        }

        step_check
    }

    /// Clear the hover index and call the function of the widget
    fn hover_out(&mut self, internal: &mut WidgetInternal<T>) {
        if let Some(id) = self.mouse_id.take() {
            let w_internal = &mut self.widgets_i[id];

            // HOVER | GRAB | PREV_LAYOUT
            self.widgets[id].hover_out(w_internal);
            internal.on(w_internal.drain(REACTIVE, 0b10_01100000));

            if internal.check(PREV_LAYOUT) {
                internal.off_on(PREV_LAYOUT, PARTIAL_TURN);
            }
        }
    }

    /// Clear the focus index and call the function of the widget
    fn focus_out(&mut self, internal: &mut WidgetInternal<T>) {
        if let Some(id) = self.focus_id.take() {
            let w_internal = &mut self.widgets_i[id];

            self.widgets[id].focus_out(w_internal);
            internal.on(w_internal.drain(REACTIVE, DRAIN_FOCUS));

            if internal.check(PREV_LAYOUT) {
                internal.off_on(PREV_LAYOUT, PARTIAL_TURN);
            }
        }
    }

    /// Get minimum dimensions provided by the Layout
    fn min_dimensions(&self) -> Dimensions<T> {
        self.layout.min_dimensions(&self.widgets_i)
    }
}