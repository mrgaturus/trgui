use crate::widget::{Widget, WidgetInternal, Boundaries};
use crate::state::{MouseState, KeyState};

type WidgetList = Vec<Box<dyn Widget>>;

pub struct Container {
    widgets: WidgetList,
    internal: WidgetInternal,
    layout: Option<Box<dyn Layout>>
}

pub trait Layout {
    fn layout(&self, container: &mut WidgetList);
    fn set_container(&self, container: &Container);
    fn get_container(&self) -> &Container;
}

impl Container {
    pub fn new() -> Self {
        Container {
            widgets: WidgetList::new(),
            internal: WidgetInternal::new((0, 0, 0, 0)),
            layout: Option::None
        }
    }

    pub fn new_with_layout(layout: Box<dyn Layout>) -> Self {
        Container {
            widgets: WidgetList::new(),
            internal: WidgetInternal::new((0, 0, 0, 0)),
            layout: Option::Some(layout)
        }
    }

    pub fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = Option::Some(layout);
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}

impl Widget for Container {
    fn draw(&self) {
        for widget in self.widgets.iter() {
            (*widget).draw();
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

    fn handle(&mut self, mouse: &MouseState, key: &KeyState) {
        let mut relative = mouse.clone();
        relative.set_relative(self.get_bounds());

        for widget in self.widgets.iter_mut() {
            (*widget).handle(&relative, key);
        }
    }

    fn get_bounds(&self) -> Boundaries {
        self.internal.boundaries()
    }

    fn set_bounds(&mut self, bounds: Boundaries) {
        self.internal.set_boundaries(bounds);
    }
}