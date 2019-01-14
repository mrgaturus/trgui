# TRGUI
My approach for a GUI Toolkit on Rust. Based on Widget traits and Containers that can handle trait objects.

## Simple test or example
```rust
extern crate trgui;
use trgui::widget::{Widget, WidgetBounds};
use trgui::window::Window;
use trgui::widgets::button::Button;

fn main() {
    // Creating a window that has a root container and event states.
    let mut window = Window::new();
    window.set_bounds((0, 0, 200, 200));
    
    // Button is a struct that implements Widget traits
    let button = Button::new("BUTTON", (5, 5, 10, 10));
    let button2 = Button::new("BUTTON TWO", (50, 50, 10, 10));
    let button3 = Button::new("BUTTON THREE", (100, 100, 10, 10));

    // Moving buttons into the root container
    window.root_container_mut().add_widget(Box::new(button));
    window.root_container_mut().add_widget(Box::new(button2));
    window.root_container_mut().add_widget(Box::new(button3));

    // Get a reference of mouse state
    let mouse_state = window.get_state_mut().0;
    // Change mouse state
    mouse_state.set_clicked(true);
    mouse_state.set_mouse((109, 109),0); // mouse clicked on button three area

    // Handle states
    window.handle_itself();
    // Draw every widget from the container
    // Containers can be nested
    window.draw();
    // Draw only prints information of widget
}
```
