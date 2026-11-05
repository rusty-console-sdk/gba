//! Fills the screen with a color and swaps it while A is held.

#![no_std]
#![no_main]

use gba_sdk::input::{Button, Buttons};
use gba_sdk::video::{wait_for_vblank, Color, Mode3};
use gba_sdk::{entry, log, LogLevel};

entry!(main);

fn main() -> ! {
    log!(LogLevel::Info, "hello from gba-sdk");

    let mut screen = Mode3::enable();
    let mut last = None;
    loop {
        let color = if Buttons::read().held(Button::A) {
            Color::GREEN
        } else {
            Color::rgb(4, 8, 20)
        };
        if last != Some(color) {
            screen.clear(color);
            last = Some(color);
        }
        wait_for_vblank();
    }
}
