use crate::widget::{Widget, WidgetInternal, WidgetFlag, Dimensions, Boundaries};
use crate::state::{MouseState, KeyState};
use crate::layout::Layout;

type WidgetList = Vec<Box<dyn Widget>>;
pub type BoundList = Vec<WidgetInternal>;

pub struct Container {
    widgets: WidgetList,
    widgets_i: BoundList,
    layout: Box<dyn Layout>,
    focus_id: Option<usize>,
    grab_id: Option<usize>,
    hover_id: Option<usize>
}

impl Container {
    pub fn new(layout: Box<dyn Layout>) -> Self {
        Container {
            widgets: WidgetList::new(),
            widgets_i: BoundList::new(),
            focus_id: Option::None,
            grab_id: Option::None,
            hover_id: Option::None,
            layout
        }
    }

    pub fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = layout;
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>, update: bool) {
        let mut internal = WidgetInternal::new((0, 0, 0, 0), 0b00001000 | (update as u8) << 2);
        internal.set_min_dimensions(widget.compute_min());

        self.widgets.push( widget );
        self.widgets_i.push( internal );
    }

    pub fn add_widget_b(&mut self, widget: Box<dyn Widget>, bounds: Boundaries, update: bool) {
        let mut internal = WidgetInternal::new(bounds, 0b00001000 | (update as u8) << 2);
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

    fn focus_id(&mut self, n: usize) {
        if let Some(id) = self.focus_id {
            if id != n {
                let widget_i = &mut self.widgets_i[id];

                self.widgets[id].unfocus(widget_i);
                self.widgets_i[id].off(WidgetFlag::Focus);
            }
        }
        self.focus_id = Some(n);
        //println!("{:?}", self.focus_id);
    }
}

impl Widget for Container {
    fn update(&mut self, _: &mut WidgetInternal) {
        self.widgets_i.iter_mut()
            .filter(|w_internal| w_internal.get(WidgetFlag::Update))
            .zip(self.widgets.iter_mut())
            .for_each(|(w_internal, widget)| 
                widget.update(w_internal)
            );
    }

    fn update_layout(&mut self, internal: &mut WidgetInternal) {
        self.layout.layout(&mut self.widgets_i, &internal.dimensions());

        if let Some(id) = self.focus_id {
            let widget_id = &mut self.widgets_i[id];
            if !widget_id.get(WidgetFlag::Visible) {
                self.unfocus(internal);
            }
        }

        self.widgets_i.iter_mut()
            .zip(self.widgets.iter_mut())
            .for_each(|(w_internal, widget)| {
                w_internal.compute_absolute(internal.absolute_pos());
                widget.update_layout(w_internal);
            });
    }

    fn handle_mouse(&mut self, mouse: &MouseState, internal: &mut WidgetInternal) {
        if self.grab_id.is_some() || !internal.get(WidgetFlag::Grab) {
            if let Some(id) = self.grab_id {
                let widget = &mut self.widgets[id];
                let widget_i = &mut self.widgets_i[id];
                {
                    let r_coords = mouse.coordinates();
                    let i_bounds = widget_i.boundaries_abs();
                    dbg!(r_coords);
                    widget_i.set(WidgetFlag::Hover, point_on_area!(r_coords, i_bounds));
                }
                widget.handle_mouse(&mouse, widget_i);
                
                if !widget_i.get(WidgetFlag::Grab) {
                    self.grab_id = None;
                    internal.off(WidgetFlag::Grab);
                }
                if widget_i.get(WidgetFlag::Focus) {
                    self.focus_id(id);
                    internal.on(WidgetFlag::Focus);
                }
            } else {
                let widget_r = self.widgets_i.iter_mut()
                .enumerate()
                .find(|(_, w_internal)| {
                    let r_coords = mouse.coordinates();
                    let i_bounds = w_internal.boundaries_abs();

                    w_internal.can_point() && point_on_area!(r_coords, i_bounds)
                });

                if let Some( (n, w_internal) ) = widget_r {
                    self.widgets[n].handle_mouse(&mouse, w_internal);

                    if w_internal.get(WidgetFlag::Grab) {
                        self.grab_id = Some(n);
                        internal.on(WidgetFlag::Grab);
                    }
                    if w_internal.get(WidgetFlag::Focus) {
                        self.focus_id(n);
                        internal.on(WidgetFlag::Focus);
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
                        internal.set(WidgetFlag::Grab, mouse.clicked());
                    }
                }
            }
        } else {
            internal.set(WidgetFlag::Grab, mouse.clicked());
        }
    }

    fn handle_keys(&mut self, key: &KeyState, _: &mut WidgetInternal) {
        if let Some(id) = self.focus_id {
            self.widgets[id].handle_keys(key, &mut self.widgets_i[id]);
        }
    }

    /// Set Widget Bounds (x, y, width, height)
    fn compute_min(&self) -> Dimensions {
        self.layout.minimum_size(&self.widgets_i)
    }

    /// Step focus on Widget array
    fn step_focus(&mut self, back: bool, _: &mut WidgetInternal) -> bool {
        if !self.widgets.is_empty() {
            if let Some(id) = self.focus_id {
                let widget = (&mut self.widgets[id], &mut self.widgets_i[id]);

                if widget.0.step_focus(back, widget.1) {
                    return true;
                } else {
                    widget.0.unfocus(widget.1);
                    widget.1.off(WidgetFlag::Focus);
                }
            }
            let mut step = self.step(back);

            if step.0 {
                while let Some(widget) = self.widgets.get_mut(step.1) {
                    let widget_i = &mut self.widgets_i[step.1];
                    let focus = widget.step_focus(back, widget_i);

                    if focus {
                        widget_i.on(WidgetFlag::Focus);
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

    fn unhover(&mut self, _: &mut WidgetInternal) {
        if let Some(id) = self.hover_id {
            let widget_i = &mut self.widgets_i[id];

            self.widgets[id].unhover(widget_i);
            widget_i.off(WidgetFlag::Hover);
            self.hover_id = Option::None;
        }
    }

    fn unfocus(&mut self, _: &mut WidgetInternal) {
        if let Some(id) = self.focus_id {
            let widget_i = &mut self.widgets_i[id];

            self.widgets[id].unfocus(widget_i);
            widget_i.off(WidgetFlag::Focus);
            self.focus_id = Option::None;
        }
    }
}