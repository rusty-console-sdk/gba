# Getting started

gba-sdk isn't on crates.io yet, so a game lives in a checkout of the repository for now. The GBA SDK is the `gba/` directory of the [collection](../index.md). These steps build the example ROM and run it in mGBA.

## Install

You need four things:

- [rustup](https://rustup.rs). `gba/rust-toolchain.toml` picks a nightly, which rustup installs on the first `cargo` call. Nightly is only there for `-Zbuild-std`: `thumbv4t-none-eabi` ships no prebuilt `core`, so it is built from source. The code itself stays within Rust 1.90.
- [mise](https://mise.jdx.dev), which installs the rest from the root `mise.toml`: `just`, the `arm-none-eabi` assembler and GCC, `agb-gbafix`, and uv for building these docs.
- [mGBA](https://mgba.io) to run ROMs. `just gba::test` runs the SDL build (`mgba`, the `mgba-sdl` package on Debian and Ubuntu); `just gba::run` opens `mgba-qt`.
- `xvfb-run`, which gives `just gba::test` a display to run mGBA on without a screen. It comes from the `xvfb` package, separate from mGBA.

```sh
git clone https://github.com/rusty-console-sdk/sdk
cd sdk
mise install
```

The tools mise installs are only on `PATH` in a shell where mise is [activated](https://mise.jdx.dev/getting-started.html). Without that, put `mise exec --` in front of each command (`mise exec -- just gba::test hello`).

## Build and run the example

From the root of the checkout:

```sh
just gba::rom hello     # writes gba/target/roms/hello.gba
just gba::test hello    # runs it in mGBA for four seconds and checks its log
```

Inside `gba/`, the same recipes work without the prefix (`just rom hello`). `just gba::test` prints the ROM's log, which ends with `ok` when the line `hello from gba-sdk` showed up. Open `gba/target/roms/hello.gba` in any emulator to see it: the screen is dark blue, and green while A is held.

## Write your own

A game is a binary crate in the GBA workspace with two attributes, the `entry!` macro and one function. This one also logs a line, so `just gba::test` has something to check:

```rust
#![no_std]
#![no_main]

use gba_sdk::video::{wait_for_vblank, Color, Mode3};
use gba_sdk::{entry, log, LogLevel};

entry!(main);

fn main() -> ! {
    log!(LogLevel::Info, "my game started");
    let mut screen = Mode3::enable();
    screen.clear(Color::rgb(4, 8, 20));
    loop {
        wait_for_vblank();
    }
}
```

1. Copy `gba/examples/hello` to `gba/examples/<name>`, change the `name` in its `Cargo.toml` and add `"examples/<name>"` to `members` in `gba/Cargo.toml`.
2. Replace its `src/main.rs` with your program. `just gba::rom <name>` builds it, and `just gba::test <name> "my game started"` runs it and checks the log for that line.

`main` never returns; there is nothing to return to. It runs after the [runtime](runtime.md) has copied `.data` to IWRAM and zeroed `.bss`, with interrupts masked.

## Building on another machine

The ARM toolchain and emulator can live on a remote machine. `just remote gba::<recipe>` syncs the whole checkout there with `rsync`, runs the recipe through `mise exec` and copies every SDK's `target/roms/` back, the GBA's to `gba/target/roms/`; `just gba::run <name>` builds remotely and opens the ROM in a local `mgba-qt`. The host is the SSH alias `sdk-build`, or whatever `SDK_BUILD_HOST` says.
