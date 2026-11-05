# Rusty Console SDK

Rust SDKs for game consoles, one directory per console.

| Console | Directory | Crate | Status |
| --- | --- | --- | --- |
| Game Boy Advance | [`gba/`](gba/) | `gba-sdk` | A ROM boots in mGBA: mode 3, buttons, logging. Not on crates.io yet. |

More consoles will follow the GBA.

## Building

Needs rustup and [mise](https://mise.jdx.dev). One `mise install` at the root fetches the tools for every SDK, and each SDK pins its own Rust toolchain in its directory.

```sh
mise install
just                    # lists every SDK's recipes
just gba::test hello    # builds the GBA example and runs it in mGBA
```

License: BSD-3-Clause.
