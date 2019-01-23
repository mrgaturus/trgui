use crate::widget::{Widget, WidgetInternal, Dimensions};
use crate::state::{MouseState, KeyState};
use crate::layout::Layout;

type WidgetList = Vec<Box<dyn Widget>>;
pub type BoundList = Vec<WidgetInternal>;

pub struct Container {
    focus_id: Option<usize>,
    grab_id: Option<usize>,
    last_id: Option<usize>,
    layout: Box<dyn Layout>,
    widgets: WidgetList,
    widgets_i: BoundList
}

impl Container {
    pub fn new(layout: Box<dyn Layout>) -> Self {
        Container {
            focus_id: Option::None,
            grab_id: Option::None,
            last_id: Option::None,
            layout,
            widgets: WidgetList::new(),
            widgets_i: BoundList::new()
        }
    }

    pub fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = layout;
    }

    pub fn add_widget(&mut self, widget: (Box<dyn Widget>, WidgetInternal)) {
        self.widgets.push(widget.0);
        self.widgets_i.push(widget.1);
    }

    pub fn del_widget(&mut self, _position: (i32, i32)) {

    }

    pub fn get_widgets(&mut self) -> &mut WidgetList {
        &mut self.widgets
    }

    fn step(&mut self, back: bool) -> (bool, usize) {
        if let None = self.focus_id {
            let mut val = 0;
            if back {
                val = self.widgets.len() - 1;
            }
            self.focus_id = Some(val);

            (true, val)
        } else if let Some(mut id) = self.focus_id {
            if back {
                id -= 1
            } else {
                id += 1
            }
            if id >= self.widgets.len() {
                self.focus_id = None;
                (false, 0)
            } else {
                self.focus_id = Some(id);
                (true, id)
            }
        } else {
            (false, 0)
        }
    }

    fn focus_id(&mut self, n: usize) {
        if let Some(id) = self.focus_id {
            if id != n {
                self.widgets_i[id].set_focused(false);
            }
        }
        self.focus_id = Some(n);
        //println!("{:?}", self.focus_id);
    }

    fn unfocus(&mut self) {
        if let Some(id) = self.focus_id {
            self.widgets_i[id].set_focused(false);
            self.focus_id = Option::None;
        }
    }
}

impl Widget for Container {
    fn draw(&self, position: &(i32, i32), _internal: &WidgetInternal) {
        for (widget, w_internal) in self.widgets.iter().zip(self.widgets_i.iter()) {
            let absolute_widget = absolute_pos!(position, w_internal.coordinates());
            widget.draw(&absolute_widget, w_internal);
        }
    }

    fn update(&mut self, layout: bool, internal: &mut WidgetInternal) {
        if layout {
            self.layout.layout(&mut self.widgets_i, &internal.dimensions());
            let min = self.layout.minimum_size(&self.widgets_i);
            internal.set_dimensions(min.0, min.1);
        }

        for (widget, internal) in self.widgets.iter_mut().zip(self.widgets_i.iter_mut()) {
            (*widget).update(layout, internal);
        }
    }

    fn handle_mouse(&mut self, mouse: &MouseState, internal: &mut WidgetInternal) {
        let mut relative = mouse.clone();

        if !internal.grabbed() || self.grab_id.is_some() {
            if let Some(id) = self.grab_id {
                self.unhover();

                let widget_i = &mut self.widgets_i[id];
                if !point_on_area!(relative.coordinates_relative(), widget_i.boundaries()) {
                    relative.enable_relative(false);
                }
                relative.set_relative(widget_i.boundaries());
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
                let widget_r = self.widgets.iter_mut()
                    .zip(self.widgets_i.iter_mut())
                    .enumerate()
                    .find(|tuple| 
                        point_on_area!(relative.coordinates_relative(), (tuple.1).1.boundaries())             
                    );

                if let Some((n, (widget, w_internal))) = widget_r {
                    relative.set_relative(w_internal.boundaries());

                    (*widget).handle_mouse(&relative, w_internal);

                    if w_internal.grabbed() {
                        self.grab_id = Some(n);
                        internal.set_grab(true);
                    }
                    if w_internal.focused() {
                        self.focus_id(n);
                        internal.set_focused(true);
                    }

                    if let Some(id) = self.last_id {
                        if id != n {
                            self.unhover();
                        }
                    }
                    self.last_id = Some(n);
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
        for widget in self.widgets.iter_mut() {
            (*widget).handle_keys(key);
        }
    }

    /// Set Widget Bounds (x, y, width, height)
    fn get_min(&self) -> Dimensions {
        self.layout.minimum_size(&self.widgets_i)
    }

    /// Step focus on Widget array
    fn step_focus(&mut self, back: bool, _internal: &mut WidgetInternal) -> bool {
        if !self.widgets.is_empty() {
            if let Some(id) = self.focus_id {
                if self.widgets[id].step_focus(back, &mut self.widgets_i[id]) {
                    return true;
                }
            }
            let mut step = self.step(back);

            if step.0 {
                while let Some(widget) = self.widgets.get_mut(step.1) {
                    let focus = widget.step_focus(back, &mut self.widgets_i[step.1]);

                    if focus {
                        return focus;
                    } else {
                        step = self.step(back);
                        if step.0 {
                            continue;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        false
    }

    fn unhover(&mut self) {
        if let Some(id) = self.last_id {
            self.widgets[id].unhover();
            self.widgets_i[id].set_hover(false);
            self.last_id = Option::None;
        }
    }
}