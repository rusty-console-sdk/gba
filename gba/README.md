# gba

Game Boy Advance SDK for Rust, to be published as `gba-sdk` (not on crates.io yet).

Games are `no_std` Rust built for `thumbv4t-none-eabi`. A second backend through [mrustc](https://github.com/thepowersgang/mrustc) (Rust to C, then `arm-none-eabi-gcc`) is planned.

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

## Building

Needs rustup, [mise](https://mise.jdx.dev) and, to run ROMs, mGBA. From the repository root:

```sh
mise install            # just, arm-none-eabi-gcc, agb-gbafix
just gba::check         # fmt and clippy for the GBA
just gba::rom hello     # gba/target/roms/hello.gba
just gba::test hello    # runs it headless in mGBA and checks its log
```

Inside `gba/`, the same recipes work without the `gba::` prefix. The nightly toolchain in `gba/rust-toolchain.toml` installs itself on the first `cargo` call. Builds can also run on a remote machine with `just remote gba::<recipe>`; see the root `AGENTS.md`.

License: BSD-3-Clause.
