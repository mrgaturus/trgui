//! Generic Mouse and Keyboard States
//! 
//! Use this on an Event Poll provided by a Window Manager

use crate::widget::Position;
use std::ops::Sub;

/// Generic Mouse State
pub struct MouseState<P> {
    /// Is clicked?
    m_click_btn: u8,
    /// Mouse coordinates
    m_coords: Position<P>,
    /// Mouse wheel
    m_wheel: i32,
    /// Tablet Pressure Level
    t_pressure: i32,
    /// Keyboard Modifiers
    k_modifiers: u16,
}

/// Generic Key State
pub struct KeyState {
    /// Pressed keycode
    k_code: Option<i32>,
    /// Keyboard Modifiers
    k_modifiers: u16,
}

impl<P> MouseState<P>
where
    P: Sized + Copy + Clone + Default + Sub<Output = P>,
{
    /// Creates a new Mouse State with default values (all on zeroes)
    pub fn new() -> Self {
        MouseState {
            m_click_btn: 0,
            m_coords: (Default::default(), Default::default()),
            m_wheel: 0,
            t_pressure: 0,
            k_modifiers: 0,
        }
    }

    /// Click a button using a bitflag
    pub fn click_button(&mut self, click: u8) {
        self.m_click_btn |= click;
    }

    /// Release a button using a bitflag
    pub fn release_button(&mut self, click: u8) {
        self.m_click_btn = self.m_click_btn & !click;
    }

    /// Set cursor position
    pub fn set_mouse(&mut self, coords: Position<P>) {
        self.m_coords = coords;
    }

    /// Set tablet pressure (optional)
    pub fn set_pressure(&mut self, pressure: i32) {
        self.t_pressure = pressure;
    }

    /// Replace all modifiers
    pub fn set_modifiers(&mut self, mods: u16) {
        self.k_modifiers = mods
    }

    /// Press modifiers using a bitflag
    pub fn press_modifiers(&mut self, mods: u16) {
        self.k_modifiers |= mods;
    }

    /// Release modifiers using a bitflag
    pub fn release_modifiers(&mut self, mods: u16) {
        self.k_modifiers = self.k_modifiers & !mods;
    }

    /// Get Absolute position of the cursor
    #[inline]
    pub fn absolute_pos(&self) -> Position<P> {
        self.m_coords
    }
    
    /// Calculate and get a relative position of the cursor with an absolute position of a widget.
    #[inline]
    pub fn relative_pos(&self, pos: Position<P>) -> Position<P> {
        (self.m_coords.0 - pos.0, self.m_coords.1 - pos.1)
    }

    /// Check the tablet pressure
    pub fn tablet_pressure(&self) -> i32 {
        self.t_pressure
    }

    /// Get clicked buttons
    pub fn clicked_buttons(&self, click: u8) -> bool {
        click & self.m_click_btn == click
    }

    /// Check if the mouse is clicked in any button
    #[inline]
    pub fn clicked(&self) -> bool {
        self.m_click_btn > 0
    }

    /// Get pressed modifiers
    #[inline]
    pub fn pressed_modifiers(&self, mods: u16) -> bool {
        mods & self.k_modifiers == mods
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

    /// Replace all modifiers
    pub fn set_modifiers(&mut self, mods: u16) {
        self.k_modifiers = mods
    }

    /// Press modifiers using a bitflag
    pub fn press_modifiers(&mut self, mods: u16) {
        self.k_modifiers |= mods;
    }

    /// Release modifiers using a bitflag
    pub fn release_modifiers(&mut self, mods: u16) {
        self.k_modifiers = self.k_modifiers & !mods;
    }

    /// Get pressed keycode
    #[inline]
    pub fn keycode(&self) -> Option<i32> {
        self.k_code
    }

    /// Check if the keyboard is pressed
    pub fn pressed(&self) -> bool {
        self.k_code.is_some()
    }

    /// Get pressed modifiers
    #[inline]
    pub fn pressed_modifiers(&self, mods: u16) -> bool {
        mods & self.k_modifiers == mods
    }
}
