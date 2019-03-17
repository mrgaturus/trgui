use crate::widget::{Widget, WidgetInternal, Dimensions, Boundaries};
use crate::widget::flags::*;
use crate::state::{MouseState, KeyState};
use crate::decorator::{Decorator, DECORATOR_BEFORE, DECORATOR_AFTER, DECORATOR_UPDATE};
use crate::layout::Layout;

type WidgetList = Vec<Box<dyn Widget>>;
pub type InternalList = Vec<WidgetInternal>;

pub struct Container {
    widgets_i: InternalList,
    widgets: WidgetList,
    decorator: Box<dyn Decorator>,
    layout: Box<dyn Layout>,
    focus_id: Option<usize>,
    grab_id: Option<usize>,
    hover_id: Option<usize>
}

impl Container {
    pub fn new(decorator: Box<dyn Decorator>, layout: Box<dyn Layout>) -> Self {
        Container {
            widgets_i: InternalList::new(),
            widgets: WidgetList::new(),
            focus_id: Option::None,
            grab_id: Option::None,
            hover_id: Option::None,
            decorator,
            layout
        }
    }

    pub fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = layout;
    }

    pub fn set_decorator(&mut self, decorator: Box<dyn Decorator>) {
        self.decorator = decorator;
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>, flags: u16) {
        let mut internal = WidgetInternal::new((0, 0, 0, 0), flags);
        internal.set_min_dimensions(widget.compute_min());

        self.widgets_i.push( internal );
        self.widgets.push( widget );
    }

    pub fn add_widget_b(&mut self, widget: Box<dyn Widget>, bounds: Boundaries, flags: u16) {
        let mut internal = WidgetInternal::new(bounds, flags);
        internal.set_min_dimensions(widget.compute_min());

        self.widgets_i.push( internal );
        self.widgets.push( widget );
    }

    fn step(&mut self, back: bool) -> (bool, usize) {
        match self.focus_id {
            Some(mut id) => {
                let len = self.widgets.len() - 1;

                if back {
                    if id == 0 {
                        self.focus_id = None;
                        return (false, len);
                    }
                    id -= 1;
                } else {
                    if id == len {
                        self.focus_id = None;
                        return (false, 0);
                    }
                    id += 1;
                }

                self.focus_id = Some(id);
                (true, id)
            },
            None => {
                let mut val = 0;
                if back {
                    val = self.widgets.len() - 1;
                }
                self.focus_id = Some(val);

                (true, val)
            }
        }
    }
}

impl Widget for Container {
    fn draw(&mut self, internal: &WidgetInternal) -> bool {
        let mut count: usize = 0;

        if internal.check(DECORATOR_BEFORE) {
            self.decorator.before(internal);
        }

        self.widgets_i.iter_mut()
            .filter(|w_internal| w_internal.check(DRAW) )
            .zip(self.widgets.iter_mut())
            .for_each(|(w_internal, widget)| {
                count += 1;

                if !widget.draw(w_internal) {
                    w_internal.off(DRAW);
                    count -= 1;
                }
            });

        if internal.check(DECORATOR_AFTER) {
            self.decorator.after(internal);
        }

        count > 0
    }

    fn update(&mut self, internal: &mut WidgetInternal, bind: bool) {
        let check_flag = if bind { UPDATE | UPDATE_BIND } else { UPDATE };
        let mut count: usize = 0;

        self.widgets_i.iter_mut()
            .filter(|w_internal| w_internal.check_any(check_flag) )
            .zip(self.widgets.iter_mut())
            .for_each(|(w_internal, widget)| {
                let backup = w_internal.val(FOCUS | GRAB | HOVER);
                widget.update(w_internal, bind);

                count += w_internal.check(UPDATE) as usize;
                if w_internal.changed() {
                    internal.replace(w_internal.val(DRAW));
                    
                    w_internal.replace(backup);
                    w_internal.unchange();
                }
            });

        if let Some(id) = self.focus_id {
            if !self.widgets_i[id].check(ENABLED | VISIBLE) {
                self.unfocus(internal);
            }
        }

        internal.set(UPDATE, count > 0);
    }

    fn update_layout(&mut self, internal: &mut WidgetInternal) {
        self.layout.layout(&mut self.widgets_i, &internal.dimensions());

        if let Some(id) = self.focus_id {
            let widget_id = &mut self.widgets_i[id];
            if !widget_id.check(VISIBLE) {
                self.unfocus(internal);
            }
        }

        self.widgets_i.iter_mut()
            .zip(self.widgets.iter_mut())
            .for_each(|(w_internal, widget)| {
                w_internal.compute_absolute(internal.absolute_pos());
                widget.update_layout(w_internal);

                if w_internal.changed() {
                    internal.replace(w_internal.val(DRAW | UPDATE));
                }
            });

        if internal.check(DECORATOR_UPDATE) {
            self.decorator.update(internal);
        }
    }

    fn handle_mouse(&mut self, internal: &mut WidgetInternal, mouse: &MouseState) {
        if self.grab_id.is_some() || !internal.check(GRAB) {
            let widget_n = self.grab_id
                .or(self.hover_id.filter(|n| {
                        self.widgets_i[*n].on_area(mouse.coordinates())
                    })
                )
                .or_else(|| {
                    self.widgets_i.iter_mut()
                        .enumerate()
                        .find_map(|(n, w_internal)| {
                            if w_internal.on_area(mouse.coordinates()) {
                                Some(n)
                            } else {
                                None
                            }
                        })
                });

            if let Some(n) = widget_n {
                let w_internal = unsafe { 
                    &mut *(self.widgets_i.get_unchecked_mut(n) as *mut WidgetInternal)
                };

                if w_internal.check(GRAB) {
                    w_internal.set(HOVER, w_internal.on_area(mouse.coordinates()))
                } else {
                    if let Some(id) = self.hover_id {
                        if id != n {
                            self.unhover(internal);
                        }
                    }
                    self.hover_id = Some(n);
                    w_internal.on(HOVER);
                }
                w_internal.unchange();

                self.widgets[n].handle_mouse(w_internal, mouse);

                if w_internal.changed() {
                    internal.replace(w_internal.val(DRAW | UPDATE));

                    internal.set(GRAB, w_internal.check(GRAB));
                    self.grab_id = Some(n).filter(|_| {
                        w_internal.check(GRAB)
                    });

                    if w_internal.check(FOCUS) {
                        if let Some(id) = self.focus_id {
                            if id != n {
                                self.unfocus(internal);
                            }
                        }

                        self.focus_id = Some(n);
                        internal.on(FOCUS);
                    }
                }
            } else {
                self.unhover(internal);

                if self.grab_id.is_none() {
                    internal.set(GRAB, mouse.clicked());
                }
            }
        } else {
            internal.set(GRAB, mouse.clicked());
        }
    }

    fn handle_keys(&mut self, internal: &mut WidgetInternal, key: &KeyState) {
        if let Some(id) = self.focus_id {
            let w_internal = &mut self.widgets_i[id];

            self.widgets[id].handle_keys(w_internal, key);
            
            if w_internal.changed() {
                internal.replace(w_internal.val(DRAW | UPDATE));

                if w_internal.check(GRAB) {
                    self.grab_id = Some(id);
                    internal.on(GRAB);
                }

                if !w_internal.check(FOCUS | ENABLED | VISIBLE) {
                    self.unfocus(internal);
                }
            }
        }
    }

    /// Set Widget Bounds (x, y, width, height)
    fn compute_min(&self) -> Dimensions {
        self.layout.minimum_size(&self.widgets_i)
    }

    /// Step focus on Widget array
    fn step_focus(&mut self, internal: &mut WidgetInternal, back: bool) -> bool {
        if !self.widgets.is_empty() {
            if let Some(id) = self.focus_id {
                let widget = (&mut self.widgets_i[id], &mut self.widgets[id]);
                let focus = widget.1.step_focus(widget.0, back);

                if widget.0.changed() {
                    internal.replace(widget.0.val(DRAW | UPDATE));
                }

                if focus {
                    return true;
                } else {
                    widget.1.unfocus(widget.0);
                    if widget.0.changed() {
                        internal.replace(widget.0.val(DRAW | UPDATE));
                    }

                    widget.0.off(FOCUS);
                }
            }
            let mut step = self.step(back);

            if step.0 {
                while let Some(widget) = self.widgets.get_mut(step.1) {
                    let w_internal = &mut self.widgets_i[step.1];
                    let focus = widget.step_focus(w_internal, back);

                    if w_internal.changed() {
                        internal.replace(w_internal.val(DRAW | UPDATE));
                    }

                    if focus && w_internal.check(ENABLED | VISIBLE) {
                        w_internal.on(FOCUS);
                        return focus;
                    } else {
                        step = self.step(back);
                        if !step.0 {
                            break;
                        }
                    }
                }
            }
        }

        false
    }

    fn unhover(&mut self, internal: &mut WidgetInternal) {
        if let Some(id) = self.hover_id {
            let w_internal = &mut self.widgets_i[id];

            self.widgets[id].unhover(w_internal);
            if w_internal.changed() {
                internal.replace(w_internal.val(DRAW | UPDATE));
            }

            w_internal.off(HOVER);
            self.hover_id = Option::None;
        }
    }

    fn unfocus(&mut self, internal: &mut WidgetInternal) {
        if let Some(id) = self.focus_id {
            let w_internal = &mut self.widgets_i[id];

            self.widgets[id].unfocus(w_internal);
            if w_internal.changed() {
                internal.replace(w_internal.val(DRAW | UPDATE));
            }

            w_internal.off(FOCUS);
            self.focus_id = Option::None;
        }
    }

    #[inline]
    fn boxed(mut self) -> Box<Self> where Self: Sized {
        self.widgets.shrink_to_fit();
        self.widgets_i.shrink_to_fit();

        Box::new(self)
    }
}