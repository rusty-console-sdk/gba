# Logging and panics

The GBA has no console. mGBA offers one: a set of registers at `0x04FFF600` that a ROM writes a string to, and that mGBA prints to its log. `log!` uses it, with the same arguments as `println!`.

```rust
use gba_sdk::{log, LogLevel};

log!(LogLevel::Info, "frame {} at {:?}", frame, position);
```

The levels are mGBA's: `Fatal`, `Error`, `Warn`, `Info` and `Debug`. A line is at most 255 bytes; longer text is cut.

On hardware, and in emulators without the interface, `log!` costs one register write and one read and prints nothing. `MgbaLog::new` does that probe (writing `0xC0DE` to the enable register and reading back `0x1DEA`), so the formatting runs only when something will receive it.

To see the log, run mGBA with logging to stdout and the `gba.debug` category on, which is what `just test` does:

```sh
mgba -C logToStdout=1 -C logLevel.gba.debug=127 target/roms/hello.gba
```

In `mgba-qt`, the log is under Tools, Logs.

## Panics

Panics abort; there is no unwinder. The handler in the [runtime](runtime.md) writes the panic message (with its file and line) to the log at `Fatal` and loops forever, so a panicking ROM shows the last frame drawn and the log says why. `debug = true` is on in the release profile so the ELF keeps line information for a debugger.

Anything that can panic in a hot path (`Mode3::set` outside the screen, an index into a slice, integer overflow in debug builds) should be checked before it gets there; the checks are what makes the API safe, and the panic is the last resort.
