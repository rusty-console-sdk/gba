# Development

The GBA SDK is the `gba/` directory of the monorepo. Paths on this page are relative to `gba/` unless they say otherwise.

## Toolchain

`rust-toolchain.toml` pins `nightly-2026-10-15` with `rust-src`, `clippy` and `rustfmt`. Nightly is there for `-Zbuild-std` only; the code uses nothing past Rust 1.90, which is the newest `rustc` source [mrustc](backends.md#mrustc) is tested against, and every manifest says `rust-version = "1.90"`.

The root `mise.toml` pins the rest, for every SDK in the collection. The GBA uses:

| Tool | Version | Role |
| --- | --- | --- |
| `just` | 1.59.0 | Runs the recipes. |
| `arm-none-eabi-gcc` (xpack) | 15.2.1-1.1 | The assembler for both backends; the C compiler and linker for mrustc. The native backend links with `rust-lld`, which comes with Rust. |
| `agb-gbafix` | 0.25.0 | Fixes the ROM header. |

`mise install` at the root fetches them. mGBA comes from the system: `mgba-sdl` and `xvfb` for headless runs, `mgba-qt` to look at a ROM.

## Layout

```mermaid
flowchart TD
  hello[examples/hello] --> sdk[crates/sdk: gba-sdk]
  sdk --> rt[crates/rt: gba-sdk-rt]
  sdk --> regs[crates/regs: gba-sdk-regs]
  rt --> regs
  rt -.crt0.s, gba.ld.-> link[the link]
```

- `Cargo.toml`: the GBA's own Cargo workspace, separate from any other SDK's.
- `crates/regs`: `Reg<T>`, `Block<T, N>` and the register constants. No policy.
- `crates/rt`: `crt0.s`, `gba.ld`, `entry!`, `log!`, the panic handler. See [The runtime](runtime.md).
- `crates/sdk`: the API. One module per hardware area.
- `examples/`: ROMs. `hello` doubles as the boot test.
- `tools/`: host-side scripts and programs. `run-rom.sh` runs a ROM headless in mGBA.
- `justfile`: the recipes below, loaded by the root `justfile` as the `gba` module.

At the root of the monorepo, `docs/gba/` holds these pages and `zensical.toml` configures the site.

Each manifest spells out its own fields instead of inheriting from the workspace, because minicargo (mrustc's cargo) isn't known to read workspace inheritance.

## Commands

From the root of the checkout. Inside `gba/`, drop the `gba::` prefix.

| Command | What it does |
| --- | --- |
| `just gba::check` | `cargo fmt --check` and `clippy` for the GBA target with warnings denied. |
| `just gba::fmt` | Formats the code. |
| `just gba::asm` | Assembles `crt0.s` into `target/rt/crt0.o`. The recipes below run it first. |
| `just gba::rom <name>` | Builds a ROM into `target/roms/<name>.gba` and prints its section sizes. |
| `just gba::test [name] [line]` | Builds the ROM, runs it headless in mGBA for four seconds and checks that its log contains `line`. Defaults to `hello` and its line. |
| `just remote gba::<recipe>` | Syncs the checkout to the `sdk-build` host (or `SDK_BUILD_HOST`) and runs the recipe there. Every SDK's `target/roms/` comes back, the GBA's to `gba/target/roms/`. |
| `just gba::run [name]` | Builds remotely and opens the ROM in a local `mgba-qt`. |
| `uvx "zensical==0.1.0" build --strict` | Run at the root: builds this site into `site/`. `serve` in place of `build --strict` previews it. |

## Testing

Anything that touches hardware is tested as a ROM in mGBA: the ROM logs a line, `tools/run-rom.sh` runs it under a timeout and greps for the line. `just gba::test` does that for `hello`. A test runner over libmgba, test ROMs under `tests/` and frame comparisons against goldens are planned, as is running every test on both backends.

Pure logic (fixed-point math, asset conversion) will get ordinary `cargo test` on the host, in modules that don't touch hardware. Nothing has that yet.

Emulators are not hardware. Timing-sensitive features need a run on a real GBA before they are called done.

## Docs

The site is built by [Zensical](https://zensical.org) from the root `docs/`, with the GBA's pages in `docs/gba/`. The strict build must pass, with the Zensical version that `.github/workflows/docs.yml` pins. The Docs workflow runs it on pull requests that touch the docs or either docs workflow, and on `main` the Pages workflow calls the same workflow to build the site, then publishes it. Code in the docs is compiled before it is committed. Every change to the code that changes what a page says updates the page in the same change.
