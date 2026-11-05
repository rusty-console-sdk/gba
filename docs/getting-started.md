# Getting started

gba-sdk isn't on crates.io yet, so a game lives in a checkout of the repository for now. These steps build the example ROM and run it in mGBA.

## Install

You need three things:

- [rustup](https://rustup.rs). The repository's `rust-toolchain.toml` picks a nightly, which rustup installs on the first `cargo` call. Nightly is only there for `-Zbuild-std`: `thumbv4t-none-eabi` ships no prebuilt `core`, so it is built from source. The code itself stays within Rust 1.90.
- [mise](https://mise.jdx.dev), which installs the rest from `mise.toml`: `just`, the `arm-none-eabi` assembler, linker and GCC, and `agb-gbafix`.
- [mGBA](https://mgba.io) to run ROMs. `just test` runs the SDL build (`mgba`) headless under `xvfb-run`; `just run` opens `mgba-qt`.

```sh
git clone https://github.com/rusty-console-sdk/gba
cd gba
mise install
```

## Build and run the example

```sh
just rom hello     # writes target/roms/hello.gba
just test hello    # runs it in mGBA for four seconds and checks its log
```

`just test` prints the ROM's log, which ends with `ok` when the line `hello from gba-sdk` showed up. Open `target/roms/hello.gba` in any emulator to see it: the screen is dark blue, and green while A is held.

## Write your own

A game is a binary crate in the workspace with two attributes, one macro and one function:

```rust
#![no_std]
#![no_main]

use gba_sdk::entry;
use gba_sdk::video::{wait_for_vblank, Color, Mode3};

entry!(main);

fn main() -> ! {
    let mut screen = Mode3::enable();
    screen.clear(Color::rgb(4, 8, 20));
    loop {
        wait_for_vblank();
    }
}
```

1. Copy `examples/hello` to `examples/<name>`, change the `name` in its `Cargo.toml` and add `"examples/<name>"` to `members` in the root `Cargo.toml`.
2. `just rom <name>` builds it. `just test <name> "<line>"` runs it and checks the log for a line of your choosing.

`main` never returns; there is nothing to return to. It runs after the [runtime](runtime.md) has copied `.data` to IWRAM and zeroed `.bss`, with interrupts masked.

## Building on another machine

The ARM toolchain and emulator can live on a remote machine. `just remote <recipe>` syncs the working copy there with `rsync`, runs the recipe through `mise exec` and copies `target/roms/` back; `just run <name>` builds remotely and opens the ROM in a local `mgba-qt`. The host is the SSH alias `gba-build`, or whatever `GBA_BUILD_HOST` says.
