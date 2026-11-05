//! The display: colors, the video modes and the vertical blank.

use gba_sdk_regs::{
    Block, DISPCNT, MODE3_FRAME, MODE3_FRAME_WORDS, SCREEN_HEIGHT, SCREEN_WIDTH, VCOUNT,
};

/// A 15-bit color: five bits each of red, green and blue, blue highest.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(transparent)]
pub struct Color(pub u16);

impl Color {
    /// Black.
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    /// White.
    pub const WHITE: Color = Color::rgb(31, 31, 31);
    /// Full red.
    pub const RED: Color = Color::rgb(31, 0, 0);
    /// Full green.
    pub const GREEN: Color = Color::rgb(0, 31, 0);
    /// Full blue.
    pub const BLUE: Color = Color::rgb(0, 0, 31);

    /// Makes a color from components in `0..=31`. Higher bits are dropped.
    pub const fn rgb(r: u16, g: u16, b: u16) -> Color {
        Color((r & 31) | ((g & 31) << 5) | ((b & 31) << 10))
    }
}

/// The width of the screen in pixels.
pub const WIDTH: usize = SCREEN_WIDTH;
/// The height of the screen in pixels.
pub const HEIGHT: usize = SCREEN_HEIGHT;

const DISPCNT_MODE3: u16 = 3;
const DISPCNT_BG2: u16 = 1 << 10;

/// Mode 3: a single 240 by 160 frame buffer of 15-bit colors, drawn
/// straight to VRAM with no double buffering.
pub struct Mode3 {
    frame: Block<u16, { WIDTH * HEIGHT }>,
}

impl Mode3 {
    /// Switches the display to mode 3 with background 2 on.
    pub fn enable() -> Mode3 {
        DISPCNT.write(DISPCNT_MODE3 | DISPCNT_BG2);
        Mode3 { frame: MODE3_FRAME }
    }

    /// Sets one pixel. Panics outside the screen.
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < WIDTH && y < HEIGHT, "pixel outside the screen");
        self.frame.get(y * WIDTH + x).write(color.0);
    }

    /// Fills the screen with one color.
    ///
    /// Writes two pixels per store. Still about half a frame of CPU from
    /// ROM; a DMA fill will replace it.
    pub fn clear(&mut self, color: Color) {
        let pair = u32::from(color.0) | (u32::from(color.0) << 16);
        MODE3_FRAME_WORDS.fill(pair);
    }
}

/// Blocks until the next vertical blank begins.
///
/// Polls `VCOUNT`, so it burns the CPU. Interrupt-driven waiting through
/// the BIOS comes with interrupt support.
pub fn wait_for_vblank() {
    while VCOUNT.read() >= HEIGHT as u16 {}
    while VCOUNT.read() < HEIGHT as u16 {}
}
