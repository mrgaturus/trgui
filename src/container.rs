use crate::widget::{Widget, WidgetInternal, Dimensions, Boundaries};
use crate::state::{MouseState, KeyState};
use crate::layout::Layout;

type WidgetList = Vec<Box<dyn Widget>>;
pub type InternalList = Vec<WidgetInternal>;

pub struct Container {
    widgets: WidgetList,
    widgets_i: InternalList,
    focus_id: Option<usize>,
    grab_id: Option<usize>,
    hover_id: Option<usize>,
    layout: Box<dyn Layout>
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

    pub fn add_widget(&mut self, widget: Box<dyn Widget>, update: bool) {
        let mut internal = WidgetInternal::new((0, 0, 0, 0), update);
        internal.set_min_dimensions(widget.compute_min());

        self.widgets.push(widget);
        self.widgets_i.push(internal);
    }

    pub fn add_widget_i(&mut self, widget: Box<dyn Widget>, internal: WidgetInternal) {
        self.widgets.push(widget);
        self.widgets_i.push(internal);
    }

    pub fn add_widget_b(&mut self, widget: Box<dyn Widget>, bounds: Boundaries, update: bool) {
        let mut internal = WidgetInternal::new(bounds, update);
        internal.set_min_dimensions(widget.compute_min());

        self.widgets.push(widget);
        self.widgets_i.push(internal);
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
                self.widgets[id].unfocus();
                self.widgets_i[id].set_focused(false);
            }
        }
        self.focus_id = Some(n);
        //println!("{:?}", self.focus_id);
    }
}

impl Widget for Container {
    fn draw(&self, position: &(i32, i32), _: &WidgetInternal) {
        for (widget, w_internal) in self.widgets.iter().zip(self.widgets_i.iter()) {
            if w_internal.visible() {
                let absolute_widget = absolute_pos!(position, w_internal.coordinates());
                widget.draw(&absolute_widget, w_internal);
            }
        }
    }

    fn update(&mut self, layout: bool, internal: &mut WidgetInternal) {
        if layout {
            let mut min = self.layout.minimum_size(&self.widgets_i);
            if internal.width() > min.0 {
                min.0 = internal.width();
            }
            if internal.height() > min.1 {
                min.1 = internal.height();
            }
            internal.set_dimensions(min.0, min.1);
            self.layout.layout(&mut self.widgets_i, &internal.dimensions());

            if let Some(id) = self.focus_id {
                let widget_id = &mut self.widgets_i[id];
                if !widget_id.visible() {
                    self.unfocus();
                }
            }
        }

        for (widget, internal) in self.widgets.iter_mut().zip(self.widgets_i.iter_mut()) {
            if internal.need_update() || layout {
                (*widget).update(layout, internal);
            }
        }
    }

    fn handle_mouse(&mut self, mouse: &MouseState, internal: &mut WidgetInternal) {
        let mut relative = mouse.clone();

        if self.grab_id.is_some() || !internal.grabbed() {
            if let Some(id) = self.grab_id {
                let widget_i = &mut self.widgets_i[id];
                {
                    let r_coords = relative.coordinates_relative();
                    let i_bounds = widget_i.boundaries();

                    widget_i.set_hover(point_on_area!(r_coords, i_bounds));
                    relative.set_relative(i_bounds);
                }
                self.widgets[id].handle_mouse(&relative, widget_i);
                
                if !widget_i.grabbed() {
                    self.grab_id = None;
                    internal.set_grab(false);
                }
                if widget_i.focused() {
                    self.focus_id(id);
                    internal.set_focused(true);
                }
            } else {
                let widget_r = self.widgets_i.iter_mut()
                .enumerate()
                .filter(|(_, internal)| {
                    internal.can_point()
                })
                .find(|(_, internal)| {
                    let r_coords = relative.coordinates_relative();
                    let i_bounds = internal.boundaries();

                    point_on_area!(r_coords, i_bounds)
                });

                if let Some((n, w_internal)) = widget_r {
                    relative.set_relative(w_internal.boundaries());

                    self.widgets[n].handle_mouse(&relative, w_internal);

                    if w_internal.grabbed() {
                        self.grab_id = Some(n);
                        internal.set_grab(true);
                    }
                    if w_internal.focused() {
                        self.focus_id(n);
                        internal.set_focused(true);
                    }

                    if let Some(id) = self.hover_id {
                        if id != n {
                            self.unhover();
                        }
                    }
                    self.hover_id = Some(n);
                } else {
                    self.unhover();
                    if self.grab_id.is_none() {
                        internal.set_grab(mouse.clicked());
                    }
                }
            }
        } else {
            internal.set_grab(mouse.clicked());
        }
    }

    fn handle_keys(&mut self, key: &KeyState) {
        if let Some(id) = self.focus_id {
            self.widgets[id].handle_keys(key);
        }
    }

    /// Set Widget Bounds (x, y, width, height)
    fn compute_min(&self) -> Dimensions {
        self.layout.minimum_size(&self.widgets_i)
    }

    /// Step focus on Widget array
    fn step_focus(&mut self, back: bool, internal: &mut WidgetInternal) -> bool {
        if !self.widgets.is_empty() && internal.visible() {
            if let Some(id) = self.focus_id {
                let widget = (&mut self.widgets[id], &mut self.widgets_i[id]);

                if widget.0.step_focus(back, widget.1) {
                    return true;
                } else {
                    widget.0.unfocus();
                    widget.1.set_focused(false);
                }
            }
            let mut step = self.step(back);

            if step.0 {
                while let Some(widget) = self.widgets.get_mut(step.1) {
                    let widget_i = &mut self.widgets_i[step.1];
                    let focus = widget.step_focus(back, widget_i);

                    if focus {
                        widget_i.set_focused(true);
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

    fn unhover(&mut self) {
        if let Some(id) = self.hover_id {
            self.widgets[id].unhover();
            self.widgets_i[id].set_hover(false);
            self.hover_id = Option::None;
        }
    }

    fn unfocus(&mut self) {
        if let Some(id) = self.focus_id {
            self.widgets[id].unfocus();
            self.widgets_i[id].set_focused(false);
            self.focus_id = Option::None;
        }
    }
}