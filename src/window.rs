use crate::state::{KeyState, MouseState};
use crate::widget::{Widget, WidgetInternal, Boundaries, Dimensions};
use crate::container::Container;

pub trait WindowBackend {
    fn show(&mut self);
    fn hide(&mut self);
    fn resize(&mut self, dimensions: Dimensions);
    fn close(&mut self);
}

pub struct Window {
    //backend: Box<dyn WindowBackend>,
    root_container: Box<Container>,
    internal: WidgetInternal,
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

    pub fn handle_all_itself(&mut self) {
        self.handle_keys();
        self.handle_mouse_itself();
    }

    pub fn handle_mouse_itself(&mut self) {
        self.root_container.handle_mouse(&self.mouse_s);
    }

    pub fn handle_keys(&mut self) {
        self.root_container.handle_keys(&self.key_s);
    }

    pub fn next_focus(&mut self) {
        self.root_container.step_focus(false);
    }

    pub fn prev_focus(&mut self) {
        self.root_container.step_focus(true);
        println!("{}", "----");
    }

    pub fn draw_window(&self) {
        self.root_container.draw(&(0, 0));
    }

    pub fn update_window(&mut self) {
        self.root_container.update();
    }

    pub fn get_bounds(&self) -> Boundaries {
        self.internal.boundaries()
    }

    pub fn get_state(&self) -> (&MouseState, &KeyState) {
        (&self.mouse_s, &self.key_s)
    }

    pub fn get_state_mut(&mut self) -> (&mut MouseState, &mut KeyState) {
        (&mut self.mouse_s, &mut self.key_s)
    }

    pub fn set_dimensions(&mut self, dimensions: Dimensions) {
        self.internal.set_dimensions(dimensions.0, dimensions.1);
        self.root_container.set_bounds((0, 0, dimensions.0, dimensions.1));
    }
}