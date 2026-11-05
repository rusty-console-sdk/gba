//! The buttons.

use gba_sdk_regs::KEYINPUT;

/// One button.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum Button {
    /// The A button.
    A = 1 << 0,
    /// The B button.
    B = 1 << 1,
    /// Select.
    Select = 1 << 2,
    /// Start.
    Start = 1 << 3,
    /// Right on the pad.
    Right = 1 << 4,
    /// Left on the pad.
    Left = 1 << 5,
    /// Up on the pad.
    Up = 1 << 6,
    /// Down on the pad.
    Down = 1 << 7,
    /// The right shoulder button.
    R = 1 << 8,
    /// The left shoulder button.
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
