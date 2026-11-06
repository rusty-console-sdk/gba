# Rusty Console SDK

Rust SDKs for game consoles. Each console gets its own SDK, built from one repository, with an API games are written against. On consoles without an operating system, like the GBA, the SDK also brings the startup code and typed access to the hardware, and games are `no_std` Rust.

| Console | Crate | Status |
| --- | --- | --- |
| [Game Boy Advance](gba/index.md) | `gba-sdk` | A ROM boots in mGBA: mode 3, buttons, logging. Not on crates.io yet. |

More consoles will follow the GBA: the DS, the 3DS, the PlayStation Vita, maybe the Switch, the N64, 6502 consoles and maybe the SNES. [Consoles](consoles.md) has the plan.

## The repository

Every SDK lives in its own directory of [rusty-console-sdk/sdk](https://github.com/rusty-console-sdk/sdk), with its own Cargo workspace and Rust toolchain. The root ties them together:

- `mise install` at the root fetches the tools for every SDK.
- `just` at the root lists every SDK's recipes. Each SDK is a module named after its directory, so the GBA's are `just gba::check`, `just gba::test` and so on.
- `just remote <sdk>::<recipe>` runs a recipe on a build machine over SSH and copies the ROMs back.

```sh
git clone https://github.com/rusty-console-sdk/sdk
cd sdk
mise install
just gba::test hello
```

Start with the GBA's [getting started](gba/getting-started.md) page.
