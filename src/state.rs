use crate::widget::Position;
use std::ops::Sub;

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

/// A key state
pub struct KeyState {
    /// Is a key from keyboard pressed?
    k_pressed: bool,
    /// Keycode pressed
    k_code: i32,
    /// Modifier (bitflag)
    k_modifiers: u8,
}

impl<P> MouseState<P>
where
    P: Sized + Copy + Clone + Default + Sub<Output = P>,
{
    pub fn new() -> Self {
        MouseState {
            m_click: false,
            m_coords: (Default::default(), Default::default()),
            m_wheel: 0,
            t_pressure: 0,
            k_modifiers: 0,
        }
    }

    pub fn set_clicked(&mut self, clicked: bool) {
        self.m_click = clicked;
    }

    pub fn set_mouse(&mut self, coords: Position<P>, pressure: i32) {
        self.m_coords = coords;
        self.t_pressure = pressure;
    }

    pub fn clicked(&self) -> bool {
        self.m_click
    }

    #[inline]
    pub fn absolute_pos(&self) -> Position<P> {
        self.m_coords
    }

    #[inline]
    pub fn relative_pos(&self, pos: Position<P>) -> Position<P> {
        (self.m_coords.0 - pos.0, self.m_coords.1 - pos.1)
    }

    pub fn tablet_pressure(&self) -> i32 {
        self.t_pressure
    }
}

impl KeyState {
    pub fn new() -> Self {
        KeyState {
            k_pressed: false,
            k_code: 0,
            k_modifiers: 0,
        }
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.k_pressed = pressed;
    }

    pub fn set_keys(&mut self, code: i32) {
        self.k_code = code;
    }

    pub fn set_modifier(&mut self, modifiers: u8) {
        self.k_modifiers = modifiers
    }

    pub fn keys(&self) -> (i32, u8) {
        (self.k_code, self.k_modifiers)
    }

    pub fn pressed(&self) -> bool {
        self.k_pressed
    }
}
