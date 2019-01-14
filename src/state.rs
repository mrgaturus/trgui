/// A mouse state
pub struct MouseState {
    /// Is clicked?
    m_click: bool,
    /// Mouse coordinates
    m_coords: (usize, usize),
    /// Tablet Pressure Level
    t_pressure: usize
}

/// A key state
pub struct KeyState {
    /// Is a key from keyboard pressed?
    k_pressed: bool,
    /// Keycode pressed
    k_code: usize,
    /// Modifier (bitflag)
    k_modifiers: u8
}

impl MouseState {
    pub fn new() -> Self {
        MouseState {
            m_click: false,
            m_coords: (0,0),
            t_pressure: 0
        }
    }

    pub fn set_clicked(&mut self, clicked: bool) {
        self.m_click = clicked;
    }

    pub fn set_mouse(&mut self, coords: (usize, usize), pressure: usize) {
        self.m_coords = coords;
        self.t_pressure = pressure;
    }

    pub fn clicked(&self) -> bool {
        self.m_click
    }

    pub fn coordinates(&self) -> (usize, usize) {
        self.m_coords
    }

    pub fn tablet_pressure(&self) -> usize {
        self.t_pressure
    }
}

impl KeyState {
    pub fn new() -> Self {
        KeyState {
            k_pressed: false,
            k_code: 0,
            k_modifiers: 0
        }
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.k_pressed = pressed;
    }

    pub fn set_keys(&mut self, code: usize, modifiers: u8) {
        self.k_code = code;
        self.k_modifiers = modifiers;
    }

    pub fn keys(&self) -> (usize, u8) {
        (self.k_code, self.k_modifiers)
    }

    pub fn pressed(&self) -> bool {
        self.k_pressed
    }
}