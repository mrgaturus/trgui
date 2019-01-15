use crate::state::{KeyState, MouseState};
use crate::widget::{Widget, WidgetInternal};
use crate::container::Container;

pub trait WindowBackend {
    fn show(&mut self);
    fn hide(&mut self);
    fn resize(&mut self);
    fn close(&mut self);
}

pub struct Window {
    //backend: Box<dyn WindowBackend>,
    root_container: Box<Container>,
    internal: WidgetInternal,
    // unsafe for now
    grab: Option<*mut Widget>,
    mouse_s: MouseState,
    key_s: KeyState
}

impl Window {
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Window {
            //backend,
            root_container: Box::new(Container::new()),
            internal: WidgetInternal::new((0, 0, 0, 0)),
            grab: Option::None,
            mouse_s: MouseState::new(),
            key_s: KeyState::new()
        }
    }

    pub fn root_container(&self) -> &Container {
        &self.root_container
    }

    pub fn root_container_mut(&mut self) -> &mut Container {
        &mut self.root_container
    }

    pub fn handle_itself(&mut self) {
        self.root_container.handle(&self.mouse_s, &self.key_s);
    }

    pub fn get_state(&self) -> (&MouseState, &KeyState) {
        (&self.mouse_s, &self.key_s)
    }

    pub fn get_state_mut(&mut self) -> (&mut MouseState, &mut KeyState) {
        (&mut self.mouse_s, &mut self.key_s)
    }
}

impl Widget for Window {
    fn draw(&self) {
        self.root_container.draw();
    }

    fn update(&mut self) {
        self.root_container.update();
    }

    // A window can handle other states outside from himself
    fn handle(&mut self, mouse: &MouseState, key: &KeyState) {
        self.root_container.handle(mouse, key);
    }

    fn get_bounds(&self) -> (i32, i32, i32, i32) {
        self.internal.boundaries()
    }

    fn set_bounds(&mut self, bounds: (i32, i32, i32, i32)) {
        self.internal.set_boundaries(bounds);
        self.root_container.set_bounds(bounds);
    }
}