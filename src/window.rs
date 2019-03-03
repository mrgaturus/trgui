use crate::state::{KeyState, MouseState};
use crate::widget::{Widget, WidgetInternal, Dimensions, flags::{DRAW, UPDATE}};
use crate::container::Container;
use crate::layout::{Layout, FixedLayout};

pub struct Window {
    //backend: Box<dyn WindowBackend>,
    root_container: Box<Container>,
    internal: WidgetInternal,
    mouse_s: MouseState,
    key_s: KeyState
}

impl Window {
    pub fn new() -> Self {
        Window {
            //backend,
            root_container: Container::new(FixedLayout::new(false).boxed()).boxed(),
            internal: WidgetInternal::new((0, 0, 0, 0), 0b00001000),
            mouse_s: MouseState::new(),
            key_s: KeyState::new()
        }
    }

    pub fn container(&self) -> &Container {
        &self.root_container
    }

    pub fn container_mut(&mut self) -> &mut Container {
        &mut self.root_container
    }

    pub fn handle_all(&mut self) {
        self.handle_keys();
        self.handle_mouse();
    }

    pub fn handle_mouse(&mut self) {
        self.root_container.handle_mouse(&self.mouse_s, &mut self.internal);
    }

    pub fn handle_keys(&mut self) {
        self.root_container.handle_keys(&self.key_s, &mut self.internal);
    }

    pub fn next_focus(&mut self) {
        if !self.root_container.step_focus(false, &mut self.internal) {
            self.root_container.step_focus(false, &mut self.internal);
        };
    }

    pub fn prev_focus(&mut self) {
        if !self.root_container.step_focus(true, &mut self.internal) {
            self.root_container.step_focus(true, &mut self.internal);
        };
    }

    pub fn update_window(&mut self) {
        if self.internal.check(UPDATE) {
            let status = self.root_container.update(&mut self.internal);
            self.internal.set(UPDATE, status);
        }
    }

    pub fn draw_window(&mut self) {
        if self.internal.check(DRAW) {
            let status = self.root_container.draw(&mut self.internal);
            self.internal.set(DRAW, status);
        }
        
    }

    pub fn update_layout(&mut self) {
        self.root_container.update_layout(&mut self.internal);
    }

    pub fn internal(&self) -> &WidgetInternal {
        &self.internal
    }

    pub fn state(&self) -> (&MouseState, &KeyState) {
        (&self.mouse_s, &self.key_s)
    }

    pub fn state_mut(&mut self) -> (&mut MouseState, &mut KeyState) {
        (&mut self.mouse_s, &mut self.key_s)
    }

    pub fn set_dimensions(&mut self, dimensions: Dimensions) {
        self.root_container.unhover(&mut self.internal);
        self.internal.set_dimensions(dimensions);
        self.update_layout();
    }
}