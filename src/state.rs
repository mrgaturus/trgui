//! Generic Mouse and Keyboard States
//! 
//! Use this on an Event Poll provided by a Window Manager

use crate::widget::Position;
use std::ops::Sub;

/// Generic Mouse State
pub struct MouseState<P> {
    /// Is clicked?
    m_click: bool,
    /// Mouse coordinates
    m_coords: Position<P>,
    /// Mouse wheel
    m_wheel: i32,
    /// Tablet Pressure Level
    t_pressure: i32,
    /// Keyboard Modifiers
    k_modifiers: u8,
}

/// Generic Key State
pub struct KeyState {
    /// Pressed keycode
    k_code: Option<i32>,
    /// Keyboard Modifiers
    k_modifiers: u8,
}

impl<P> MouseState<P>
where
    P: Sized + Copy + Clone + Default + Sub<Output = P>,
{
    /// Creates a new Mouse State with default values (all on zeroes)
    pub fn new() -> Self {
        MouseState {
            m_click: false,
            m_coords: (Default::default(), Default::default()),
            m_wheel: 0,
            t_pressure: 0,
            k_modifiers: 0,
        }
    }

    /// Set if the mouse is clicked
    pub fn set_clicked(&mut self, clicked: bool) {
        self.m_click = clicked;
    }

    /// Set cursor position
    pub fn set_mouse(&mut self, coords: Position<P>, pressure: i32) {
        self.m_coords = coords;
        self.t_pressure = pressure;
    }

    /// Check if the mouse is clicked
    pub fn clicked(&self) -> bool {
        self.m_click
    }

    #[inline]
    /// Get Absolute position of the cursor
    pub fn absolute_pos(&self) -> Position<P> {
        self.m_coords
    }

    #[inline]
    /// Calculate and get a relative position of the cursor with an absolute position of a widget.
    pub fn relative_pos(&self, pos: Position<P>) -> Position<P> {
        (self.m_coords.0 - pos.0, self.m_coords.1 - pos.1)
    }

    /// Check the tablet pressure
    pub fn tablet_pressure(&self) -> i32 {
        self.t_pressure
    }
}

impl KeyState {
    /// Creates a new state with default values
    pub fn new() -> Self {
        KeyState {
            k_code: None,
            k_modifiers: 0,
        }
    }

    /// Set a keycode of the key pressed
    pub fn set_keycode(&mut self, code: Option<i32>) {
        self.k_code = code;
    }

    /// Set pressed modifiers (ex: shift, ctrl, alt)
    pub fn set_modifiers(&mut self, modifiers: u8) {
        self.k_modifiers = modifiers
    }

    /// Get pressed keycode
    pub fn keycode(&self) -> Option<i32> {
        self.k_code
    }

    /// Get pressed modifiers
    pub fn modifiers(&self) -> u8 {
        self.k_modifiers
    }

    /// Check if the keyboard is pressed
    pub fn pressed(&self) -> bool {
        self.k_code.is_some()
    }
}
