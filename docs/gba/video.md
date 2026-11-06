# Video

`gba_sdk::video` covers the display. Today that is colors, mode 3 and waiting for the vertical blank.

## Colors

The GBA draws 15-bit color: five bits each of red, green and blue, blue in the high bits. `Color` wraps the `u16` the hardware takes.

```rust
use gba_sdk::video::Color;

let sky = Color::rgb(4, 8, 20);   // components in 0..=31
let raw = Color(0x7FFF);          // white, as the hardware sees it
```

`Color::BLACK`, `WHITE`, `RED`, `GREEN` and `BLUE` are there for convenience. `rgb` drops bits above the lowest five, so `rgb(255, 0, 0)` is `rgb(31, 0, 0)`.

## Mode 3

Mode 3 is the simplest display mode: one 240 by 160 frame buffer of colors, drawn straight into VRAM. `WIDTH` (240) and `HEIGHT` (160) are the screen's size in pixels, as `usize`. There is no second buffer, so what you write is what the next scanline shows.

```rust
use gba_sdk::video::{Color, Mode3, HEIGHT, WIDTH};

let mut screen = Mode3::enable();
screen.clear(Color::BLACK);
for x in 0..WIDTH {
    screen.set(x, HEIGHT / 2, Color::WHITE);
}
```

`Mode3::enable` writes `DISPCNT` to pick the mode and turn background 2 on, which is where the frame buffer appears. `set` panics outside the screen. `clear` writes two pixels per store and still costs about half a frame of CPU, so call it once, not every frame; a DMA fill is on the [roadmap](roadmap.md).

Mode 3 is the mode for first programs and for software rendering. Tiled backgrounds and sprites, which is how most games draw, are planned.

## The vertical blank

The display draws 160 lines, then spends 68 lines' worth of time in the vertical blank, when VRAM can be changed without tearing. `wait_for_vblank` returns at the start of the next one.

```rust
use gba_sdk::video::wait_for_vblank;

loop {
    wait_for_vblank();
    // Draw first, while the blank lasts, then update the game for the next frame.
}
```

Drawing right after the wait gives it the whole blank. Updating first would spend part of the blank on game logic, and the drawing could still be going when the display starts on the next frame.

It polls `VCOUNT`, so it burns the CPU while waiting. Once interrupts are in, it will call the BIOS's `VBlankIntrWait` and sleep instead.
