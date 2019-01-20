use crate::widget::{Widget, WidgetInternal, Boundaries, WidgetGrab};
use crate::state::{MouseState, KeyState};

type WidgetList = Vec<Box<dyn Widget>>;

pub struct Container {
    focus_id: Option<usize>,
    grab_id: Option<usize>,
    last_id: Option<usize>,
    layout: Option<Box<dyn Layout>>,
    widgets: WidgetList,
    internal: WidgetInternal
    
}

pub trait Layout {
    fn layout(&self, container: &mut WidgetList);
    fn set_container(&self, container: &Container);
    fn get_container(&self) -> &Container;
}

impl Container {
    pub fn new() -> Self {
        Container {
            focus_id: Option::None,
            grab_id: Option::None,
            last_id: Option::None,
            layout: Option::None,
            widgets: WidgetList::new(),
            internal: WidgetInternal::new((0, 0, 0, 0))
            
        }
    }

    pub fn new_with_layout(layout: Box<dyn Layout>) -> Self {
        Container {
            focus_id: Option::None,
            grab_id: Option::None,
            last_id: Option::None,
            layout: Option::Some(layout),
            widgets: WidgetList::new(),
            internal: WidgetInternal::new((0, 0, 0, 0))
            
        }
    }

    pub fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = Option::Some(layout);
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
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
            if id > self.widgets.len() - 1 {
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
                self.widgets[id].unfocus();
            }
        }
        self.focus_id = Some(n);
        //println!("{:?}", self.focus_id);
    }
}

impl Widget for Container {
    fn draw(&self, position: &(i32, i32)) {
        for widget in self.widgets.iter() {
            let absolute_widget = absolute_pos!(position, widget.get_bounds());
            (*widget).draw(&absolute_widget);
        }
    }

    fn update(&mut self) {
        if let Some(layout) = &self.layout {
            (*layout).layout(&mut self.widgets);
        }

        for widget in self.widgets.iter_mut() {
            (*widget).update();
        }
    }

    fn handle_mouse(&mut self, mouse: &MouseState) -> (bool, bool) {
        let mut relative = mouse.clone();
        relative.set_relative(self.get_bounds());
        if !self.internal.grabbed() {
            if let Some(id) = self.grab_id {
                //println!("{} {}", "Grabbed", id);
                let widget = &mut self.widgets[id];
                
                if !point_on_area!(relative.coordinates_relative(), widget.get_bounds()) {
                    relative.enable_relative(false);
                }
                relative.set_relative_recur(widget.get_bounds());
                
                let grab = widget.handle_mouse(&relative);
                
                if !grab.1 {
                    self.grab_id = None;
                }
                if grab.0 {
                    self.focus_id(id);
                }
                return grab;
            }
            for (n, widget) in self.widgets.iter_mut().enumerate() {
                if point_on_area!(relative.coordinates_relative(), widget.get_bounds()) {
                    relative.set_relative_recur(widget.get_bounds());

                    let action = (*widget).handle_mouse(&relative);
                    if action.0 {
                        self.focus_id(n);
                    }
                    if action.1 {
                        self.grab_id = Some(n);
                    }
                    if let Some(id) = self.last_id {
                        if id != n {
                            dbg!(id);
                            self.unhover();
                        }
                    }
                    self.last_id = Some(n);
                    return action;
                }
            }
        }

        self.unhover();
        self.internal.set_grab(mouse.clicked());
        (false, self.internal.grabbed())
    }

    fn handle_keys(&mut self, key: &KeyState) {
        for widget in self.widgets.iter_mut() {
            (*widget).handle_keys(key);
        }
    }

    fn get_bounds(&self) -> Boundaries {
        self.internal.boundaries()
    }

    fn set_bounds(&mut self, bounds: Boundaries) {
        self.internal.set_boundaries(bounds);
    }

    /// Step focus on Widget array
    fn step_focus(&mut self, back: bool) -> bool {
        if !self.widgets.is_empty() {
            if let Some(id) = self.focus_id {
                if self.widgets[id].step_focus(back) {
                    return true;
                }
            }
            let mut step = self.step(back);

            if step.0 {
                while let Some(widget) = self.widgets.get_mut(step.1) {
                    let focus = widget.step_focus(back);

                    if focus {
                        return focus;
                    } else {
                        step = self.step(back);
                        continue;
                    }
                }
            }
        }

        false
    }

    fn focus(&mut self) {
        self.step_focus(false);
    }

    /// Unfocus container and everything inside
    fn unfocus(&mut self) {
        if let Some(id) = self.focus_id {
            self.widgets[id].unfocus();
            self.focus_id = Option::None;
        }
    }

    fn unhover(&mut self) {
        if let Some(id) = self.last_id {
            self.widgets[id].unhover();
        }
        self.last_id = Option::None;
    }
}

impl WidgetGrab for Container {
    /// Grab for a window state
    fn grab(&mut self) {
        if !self.internal.grabbed() {
            self.internal.set_grab(true);
        }
    }
    /// Ungrab from a window state
    fn ungrab(&mut self) {
        if self.internal.grabbed() {
            self.internal.set_grab(false);
        }
    }
}