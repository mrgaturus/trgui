use crate::widget::{Widget, WidgetInternal, Dimensions, Boundaries};
use crate::widget::flags::*;
use crate::state::{MouseState, KeyState};
use crate::layout::Layout;

type WidgetList = Vec<Box<dyn Widget>>;
pub type InternalList = Vec<WidgetInternal>;

pub struct Container {
    widgets_i: InternalList,
    widgets: WidgetList,
    layout: Box<dyn Layout>,
    focus_id: Option<usize>,
    grab_id: Option<usize>,
    hover_id: Option<usize>
}

impl Container {
    pub fn new(layout: Box<dyn Layout>) -> Self {
        Container {
            widgets_i: InternalList::new(),
            widgets: WidgetList::new(),
            focus_id: Option::None,
            grab_id: Option::None,
            hover_id: Option::None,
            layout
        }
    }

    pub fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = layout;
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

    fn update(&mut self, internal: &mut WidgetInternal, bind: bool) {
        let check_flag = if bind { UPDATE | UPDATE_BIND } else { UPDATE };
        let mut count: usize = 0;

        for ((n, w_internal), widget) in self.widgets_i.iter_mut()
            .enumerate()
            .filter(|(_, w_internal)| w_internal.check(check_flag) )
            .zip(self.widgets.iter_mut())
        {
            widget.update(w_internal, bind);

            count += w_internal.check(UPDATE) as usize;
            if w_internal.changed() {
                internal.replace(w_internal.val(DRAW));

                if w_internal.check_any(GRAB | FOCUS | HOVER) {
                    w_internal.off(GRAB | FOCUS | HOVER);

                    if let Some(id) = self.grab_id {
                        if id == n {
                            w_internal.on(GRAB);
                        }
                    }

                    if let Some(id) = self.focus_id {
                        if id == n {
                            w_internal.on(FOCUS);
                        }
                    }

                    if let Some(id) = self.hover_id {
                        if id == n {
                            w_internal.on(HOVER);
                        }
                    }

                    w_internal.unchange();
                }
            }
        }

        if count == 0 {
            internal.off(UPDATE);
        }
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
                let w_internal = &mut self.widgets_i[n];
                w_internal.set(HOVER, w_internal.on_area(mouse.coordinates()));
                w_internal.unchange();

                self.widgets[n].handle_mouse(&mouse, w_internal);
                
                if w_internal.changed() {
                    internal.replace(w_internal.val(DRAW | UPDATE));

                    if !w_internal.check(GRAB) {
                        self.grab_id = None;
                        internal.off(GRAB);
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
                }
            } else {
                let widget_r = if let Some(id) = self.hover_id.filter(|n| {
                    self.widgets_i[*n].on_area(mouse.coordinates())
                }) {
                    Some ( (id, &mut self.widgets_i[id] ) ).filter(|(_, w_internal)| {
                        w_internal.check(ENABLED)
                    })
                } else {
                    self.widgets_i.iter_mut()
                        .enumerate()
                        .find(|(_, w_internal)| {
                            w_internal.on_area(mouse.coordinates())
                        })
                        .filter(|(_, w_internal)| {
                            w_internal.check(ENABLED)
                        })
                };

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
                                }
                            }

                            self.focus_id = Some(n);
                            internal.on(FOCUS);
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
            let w_internal = &mut self.widgets_i[id];

            self.widgets[id].handle_keys(key, w_internal);
            
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
                    let w_internal = &mut self.widgets_i[step.1];
                    let focus = widget.step_focus(back, w_internal);

                    if w_internal.changed() {
                        internal.replace(w_internal.val(DRAW | UPDATE));
                    }

                    if focus {
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