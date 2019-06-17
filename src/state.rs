//! Generic Mouse and Keyboard States
//!
//! Use this to consume an Event Poll provided by a Window Manager

use crate::widget::Position;
use std::ops::Sub;

/// Generic Mouse Buttons State
pub enum MouseType {
    None,
    Wheel(f32),
    Pressed(u8),
    Released(u8),
    CursorMoved,
}

/// Generic Mouse State
pub struct MouseState<T> {
    /// Buttons clicked as bitflags
    pub m_type: MouseType,
    /// Mouse coordinates
    m_position: Position<T>,
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

impl MouseType {
    /// Check if the mouse state type is Pressed
    #[inline]
    pub fn pressed(&self) -> bool {
        match *self {
            MouseType::Pressed(_) => true,
            _ => false,
        }
    }
}

impl<T> MouseState<T>
where
    T: Sized + Copy + Clone + Default + Sub<Output = T>,
{
    /// Creates a new Mouse State with default values (all on zeroes)
    pub fn new() -> Self {
        MouseState {
            m_type: MouseType::None,
            m_position: (Default::default(), Default::default()),
            t_pressure: 0.0,
            k_modifiers: 0,
        }
    }

    /// Set cursor position
    pub fn set_position(&mut self, position: Position<T>) {
        self.m_position = position;
    }

    /// Set mouse state type
    pub fn set_type(&mut self, m_type: MouseType) {
        self.m_type = m_type;
    }

    /// Set tablet pressure (optional)
    pub fn set_pressure(&mut self, pressure: f32) {
        self.t_pressure = pressure;
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
    /// Get tablet pressure value
    pub fn tablet_pressure(&self) -> f32 {
        self.t_pressure
    }

    // Modifiers
    /// Replace all modifiers
    pub fn set_modifiers(&mut self, mods: u16) {
        self.k_modifiers = mods
    }

    /// Check if there is pressed modifiers using a bitflags mask
    #[inline]
    pub fn check_modifiers(&self, mods: u16) -> bool {
        mods & self.k_modifiers == mods
    }
}

impl KeyState {
    /// Check if there is pressed modifiers using a bitflags mask
    #[inline]
    pub fn check_modifiers(&self, mods: u16) -> bool {
        match *self {
            KeyState::Pressed(_, s_mods) | KeyState::Released(_, s_mods) => mods & s_mods == mods,
            KeyState::None => false,
        }
    }
}
