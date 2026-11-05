# The runtime

`gba-sdk-rt` is what every ROM links. It isn't used directly; `gba-sdk` depends on it and re-exports `entry!` and `log!`. This page says what it does between power-on and `main`, and where things go in memory.

## Startup

The ROM starts with a 192-byte header: an ARM branch to the entry point, then the Nintendo logo, title, codes and checksum that `agb-gbafix` fills in after the link. The entry point, in `crt0.s`, runs in ARM mode and:

1. Sets the IRQ, SVC and user stacks at the top of IWRAM (`0x03007FA0`, `0x03007FE0` and `0x03007F00`), then switches to system mode.
2. Copies `.data` from ROM to IWRAM. That section also holds any function or static marked `#[link_section = ".iwram"]`.
3. Zeroes `.bss`.
4. Jumps to `main` with `bx`, switching to Thumb.

Interrupts are enabled in CPSR but masked by `IME`, so nothing fires until the SDK turns them on.

```mermaid
flowchart LR
  bios[BIOS] --> header[Header: b entry]
  header --> stacks[Stacks] --> data[.data to IWRAM] --> bss[Zero .bss] --> main[main, Thumb]
```

## The entry point

```rust
gba_sdk::entry!(main);

fn main() -> ! {
    loop {}
}
```

`entry!` defines the `extern "C"` function the startup code jumps to and checks that yours has the type `fn() -> !`. The exported symbol is named `main` so the same startup code works on both [backends](backends.md); your function can have any name.

## Memory

The linker script `gba.ld` lays a ROM out like this:

| Section | Where | What |
| --- | --- | --- |
| `.text`, `.rodata` | ROM at `0x08000000` | Code and constants. The ROM bus is 16 bits wide with wait states, which is why the code is Thumb. |
| `.data` | IWRAM at `0x03000000`, copied from ROM | Initialized statics and everything in `.iwram*` sections. |
| `.bss` | IWRAM, after `.data` | Zeroed statics. |

IWRAM is 32 KB on a 32-bit bus with no wait states; it holds the stacks, the interrupt vector at `0x03007FFC`, and whatever fits below. The linker reserves the top 256 bytes for the IRQ and SVC stacks and the vector, and 2 KB under `0x03007F00` for the user stack, so a `.data` or `.bss` that grows into that space fails at link time rather than being overwritten at run time.

EWRAM, 256 KB at `0x02000000` on a 16-bit bus, is not placed by the script yet. It will hold the allocator's heap and an `.ewram` section.

Put hot code in IWRAM with `#[link_section = ".iwram.name"]`. It runs from the fast bus, but it still has to be Thumb for now: switching a single function to ARM mode needs `#[instruction_set]`, which the mrustc backend can't do, so ARM-mode routines go in `crt0.s` and its future siblings as assembly.

## Panics and memory functions

The panic handler is here; see [Logging and panics](logging.md). `memcpy`, `memset` and the `__aeabi_` memory functions come from `compiler_builtins` on the native backend for now. They will move into the runtime as assembly, since the mrustc backend's GCC output calls them too and they must be the same on both.
