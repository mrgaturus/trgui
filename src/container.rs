use crate::widget::{Widget, WidgetInternal, Dimensions, Boundaries};
use crate::widget::flags::*;
use crate::state::{MouseState, KeyState};
use crate::layout::Layout;

type WidgetList = Vec<Box<dyn Widget>>;
pub type InternalList = Vec<WidgetInternal>;

pub struct Container {
    widgets: WidgetList,
    widgets_i: InternalList,
    layout: Box<dyn Layout>,
    focus_id: Option<usize>,
    grab_id: Option<usize>,
    hover_id: Option<usize>
}

impl Container {
    pub fn new(layout: Box<dyn Layout>) -> Self {
        Container {
            widgets: WidgetList::new(),
            widgets_i: InternalList::new(),
            focus_id: Option::None,
            grab_id: Option::None,
            hover_id: Option::None,
            layout
        }
    }

    pub fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = layout;
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>, flags: u8) {
        let mut internal = WidgetInternal::new((0, 0, 0, 0), flags);
        internal.set_min_dimensions(widget.compute_min());

        self.widgets.push( widget );
        self.widgets_i.push( internal );
    }

    pub fn add_widget_b(&mut self, widget: Box<dyn Widget>, bounds: Boundaries, flags: u8) {
        let mut internal = WidgetInternal::new(bounds, flags);
        internal.set_min_dimensions(widget.compute_min());

        self.widgets.push( widget );
        self.widgets_i.push( internal );
    }

    pub fn del_widget(&mut self, id: usize) {
        if let Some(focus) = self.focus_id {
            if id == focus {
                self.focus_id = None;
            } else if id < focus {
                self.focus_id = Some(focus - 1);
            }
        }
        if let Some(grab) = self.grab_id {
            if id == grab {
                self.grab_id = None;
            } else if id < grab {
                self.grab_id = Some(grab - 1);
            }
        }
        if let Some(hover) = self.hover_id {
            if id == hover {
                self.hover_id = None;
            } else if id < hover {
                self.hover_id = Some(hover - 1);
            }
        }

        self.widgets.remove(id);
        self.widgets_i.remove(id);
    }

    pub fn get_widgets(&mut self) -> &mut WidgetList {
        &mut self.widgets
    }

    fn step(&mut self, back: bool) -> (bool, usize) {
        match self.focus_id {
            None => {
                let mut val = 0;
                if back {
                    val = self.widgets.len() - 1;
                }
                self.focus_id = Some(val);

                (true, val)
            },
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
            }
        }
    }
}

impl Widget for Container {
    fn draw(&mut self, _: &WidgetInternal) -> bool {
        let mut count: usize = 0;

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

        count > 0
    }

    fn update(&mut self, _: &WidgetInternal) -> bool {
        let mut count: usize = 0;

        self.widgets_i.iter_mut()
            .filter(|w_internal| w_internal.check(UPDATE) )
            .zip(self.widgets.iter_mut())
            .for_each(|(w_internal, widget)| {
                count += 1;

                if !widget.update(w_internal) {
                    w_internal.off(UPDATE);
                    count -= 1;
                }
            });

        count > 0
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
    }

    fn handle_mouse(&mut self, mouse: &MouseState, internal: &mut WidgetInternal) {
        if self.grab_id.is_some() || !internal.check(GRAB) {
            if let Some(n) = self.grab_id {
                let widget_i = &mut self.widgets_i[n];

                {
                    let r_coords = mouse.coordinates();
                    let i_bounds = widget_i.boundaries_abs();

                    widget_i.set(HOVER, point_on_area!(r_coords, i_bounds));
                }

                self.widgets[n].handle_mouse(&mouse, widget_i);
                
                if widget_i.changed() {
                    internal.replace(widget_i.val(DRAW | UPDATE));

                    if !widget_i.check(GRAB) {
                        self.grab_id = None;
                        internal.off(GRAB);
                    }
                    if widget_i.check(FOCUS) {
                        if let Some(id) = self.focus_id {
                            if id != n {
                                self.unfocus(internal);
                                internal.on(FOCUS);
                            }
                        }
                        self.focus_id = Some(n);
                    }
                }
            } else {
                let widget_r = self.widgets_i.iter_mut()
                .enumerate()
                .find(|(_, w_internal)| {
                    let r_coords = mouse.coordinates();
                    let i_bounds = w_internal.boundaries_abs();

                    point_on_area!(r_coords, i_bounds) && w_internal.check(VISIBLE)
                })
                .filter(|(_, w_internal)| {
                    w_internal.check(ENABLED)
                });

                if let Some( (n, w_internal) ) = widget_r {
                    w_internal.on(HOVER);
                    w_internal.unchange();

                    self.widgets[n].handle_mouse(&mouse, w_internal);

                    if w_internal.changed() {
                        internal.replace(w_internal.val(DRAW | UPDATE));

                        if w_internal.check(GRAB) {
                            self.grab_id = Some(n);
                            internal.on(GRAB);
                        }

                        if w_internal.check(FOCUS) {
                            if let Some(id) = self.focus_id {
                                if id != n {
                                    self.unfocus(internal);
                                    internal.on(FOCUS);
                                }
                            }
                            self.focus_id = Some(n);
                        }
                    } else {
                        w_internal.on(1);
                    }

                    if let Some(id) = self.hover_id {
                        if id != n {
                            self.unhover(internal);
                        }
                    }
                    self.hover_id = Some(n);
                } else {
                    self.unhover(internal);
                    if self.grab_id.is_none() {
                        internal.set(GRAB, mouse.clicked());
                    }
                }
            }
        } else {
            internal.set(GRAB, mouse.clicked());
        }
    }

    fn handle_keys(&mut self, key: &KeyState, internal: &mut WidgetInternal) {
        if let Some(id) = self.focus_id {
            let widget_i = &mut self.widgets_i[id];

            self.widgets[id].handle_keys(key, widget_i);
            
            if widget_i.changed() {
                internal.replace(widget_i.val(DRAW | UPDATE));

                if widget_i.check(GRAB) {
                    self.grab_id = Some(id);
                    internal.on(GRAB);
                }

                if !widget_i.check(FOCUS | ENABLED | VISIBLE) {
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
    fn step_focus(&mut self, back: bool, internal: &mut WidgetInternal) -> bool {
        if !self.widgets.is_empty() {
            if let Some(id) = self.focus_id {
                let widget = (&mut self.widgets[id], &mut self.widgets_i[id]);
                let focus = widget.0.step_focus(back, widget.1);

                if widget.1.changed() {
                    internal.replace(widget.1.val(DRAW | UPDATE));
                }

                if focus {
                    return true;
                } else {
                    widget.0.unfocus(widget.1);
                    if widget.1.changed() {
                        internal.replace(widget.1.val(DRAW | UPDATE));
                    }

                    widget.1.off(FOCUS);
                }
            }
            let mut step = self.step(back);

            if step.0 {
                while let Some(widget) = self.widgets.get_mut(step.1) {
                    let widget_i = &mut self.widgets_i[step.1];
                    let focus = widget.step_focus(back, widget_i);

                    if widget_i.changed() {
                        internal.replace(widget_i.val(DRAW | UPDATE));
                    }

                    if focus {
                        widget_i.on(FOCUS);
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
            let widget_i = &mut self.widgets_i[id];

            self.widgets[id].unhover(widget_i);
            if widget_i.changed() {
                internal.replace(widget_i.val(DRAW | UPDATE));
            }

            widget_i.off(HOVER);
            self.hover_id = Option::None;
        }
    }

    fn unfocus(&mut self, internal: &mut WidgetInternal) {
        if let Some(id) = self.focus_id {
            let widget_i = &mut self.widgets_i[id];

            self.widgets[id].unfocus(widget_i);
            if widget_i.changed() {
                internal.replace(widget_i.val(DRAW | UPDATE));
            }

            widget_i.off(FOCUS);
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