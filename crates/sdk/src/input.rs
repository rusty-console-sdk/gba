//! The buttons.

use gba_sdk_regs::KEYINPUT;

/// One button.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum Button {
    A = 1 << 0,
    B = 1 << 1,
    Select = 1 << 2,
    Start = 1 << 3,
    Right = 1 << 4,
    Left = 1 << 5,
    Up = 1 << 6,
    Down = 1 << 7,
    R = 1 << 8,
    L = 1 << 9,
}

const ALL: u16 = 0x03FF;

/// The buttons held at one moment. A set bit means held, the opposite of
/// the hardware register.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Buttons(u16);

impl Buttons {
    /// Reads the buttons now.
    pub fn read() -> Buttons {
        Buttons(!KEYINPUT.read() & ALL)
    }

    /// Whether `button` is held.
    pub fn held(self, button: Button) -> bool {
        self.0 & button as u16 != 0
    }

    /// Whether any button is held.
    pub fn any(self) -> bool {
        self.0 != 0
    }
}
