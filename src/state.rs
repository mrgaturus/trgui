//! Generic Mouse and Keyboard States
//!
//! Use this to consume an Event Poll provided by a Window Manager

use crate::widget::Position;
use std::ops::Sub;

/// Generic Mouse State
pub struct MouseState<T> {
    /// Buttons clicked as bitflags
    m_click_btn: u8,
    /// Mouse coordinates
    m_position: Position<T>,
    /// Mouse wheel
    m_wheel: f32,
    /// Tablet Pressure Level
    t_pressure: f32,
    /// Keyboard Modifiers as bitflags
    k_modifiers: u16,
}

/// Generic Key State
pub enum KeyState {
    None,
    Pressed(u32, u16),
    Released(u32, u16),
}

impl<T> MouseState<T>
where
    T: Sized + Copy + Clone + Default + Sub<Output = T>,
{
    /// Creates a new Mouse State with default values (all on zeroes)
    pub fn new() -> Self {
        MouseState {
            m_click_btn: 0,
            m_position: (Default::default(), Default::default()),
            m_wheel: 0.0,
            t_pressure: 0.0,
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
    pub fn set_mouse(&mut self, position: Position<T>) {
        self.m_position = position;
    }

    /// Set tablet pressure (optional)
    pub fn set_pressure(&mut self, pressure: f32) {
        self.t_pressure = pressure;
    }

    /// Set wheel delta
    pub fn set_wheel(&mut self, wheel: f32) {
        self.m_wheel = wheel;
    }

    #[inline]
    /// Check if requested buttons are clicked
    pub fn clicked_buttons(&self, click: u8) -> bool {
        click & self.m_click_btn == click
    }

    /// Check if the mouse is clicked by any button
    #[inline]
    pub fn clicked(&self) -> bool {
        self.m_click_btn > 0
    }

    /// Get Absolute position of the cursor
    #[inline]
    pub fn absolute_pos(&self) -> Position<T> {
        self.m_position
    }

    /// Calculate and get a relative position of the cursor with an absolute position of a widget.
    #[inline]
    pub fn relative_pos(&self, pos: Position<T>) -> Position<T> {
        (self.m_position.0 - pos.0, self.m_position.1 - pos.1)
    }

    #[inline]
    /// Get the tablet pressure value
    pub fn tablet_pressure(&self) -> f32 {
        self.t_pressure
    }

    #[inline]
    /// Get mouse delta
    pub fn wheel(&self) -> f32 {
        self.m_wheel
    }

    // Modifiers
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

    /// Get pressed modifiers
    #[inline]
    pub fn pressed_modifiers(&self, mods: u16) -> bool {
        mods & self.k_modifiers == mods
    }
}

impl KeyState {
    /// Check if there is pressed modifiers using a bitflags mask
    #[inline]
    pub fn check_modifiers(&self, mods: u16) -> bool {
        //mods & self.k_modifiers == mods
        match *self {
            KeyState::Pressed(_, s_mods) | KeyState::Released(_, s_mods) => mods & s_mods == mods,
            KeyState::None => false
        }
    }
}
