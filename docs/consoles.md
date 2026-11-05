# Consoles

The Game Boy Advance is first. This page is the plan for the consoles after it: the order, and how Rust would reach each one. None of them has any code yet.

The order puts SDK work on compilers that already exist ahead of anything that needs compiler work, and the consoles that reuse the most from earlier SDKs first.

| Order | Console | CPU | Compiler path | Main difficulty |
| --- | --- | --- | --- | --- |
| 1 | Game Boy Advance | ARM7TDMI | `rustc` for `thumbv4t-none-eabi`; mrustc and GCC as a second backend | In progress. See [the GBA pages](gba/index.md). |
| 2 | Nintendo DS | ARM946E-S and ARM7TDMI | `rustc` for the ARM targets the GBA uses, and the same GCC for mrustc | Two CPUs talking to each other, the 3D engine, the touch screen |
| 3 | Nintendo 3DS | ARM11 MPCore | `rustc` for `armv6k-nintendo-3ds`, with `std` | Fitting an SDK around Horizon OS, the GPU |
| 4 | PlayStation Vita | ARM Cortex-A9 | `rustc` for `armv7-sony-vita-newlibeabihf`, with `std` | Fitting an SDK around the Vita's OS, the GPU |
| 5 | Nintendo Switch (maybe) | ARM Cortex-A57 | `rustc` for `aarch64-nintendo-switch-freestanding`, without `std` | Testing without a dependable emulator |
| 6 | Nintendo 64 | MIPS VR4300 | `rustc` with a custom target based on `mips-mti-none-elf`; GCC for `mips64-elf` | The RSP, the graphics and sound coprocessor with its own microcode |
| 7 | PC Engine or NES | 6502 family | llvm-mos, through a Rust fork | Tiny memory, a 16-bit `usize`, bank-switched ROM |
| 8 | Super Nintendo | WDC 65816 | Unknown until a spike; possibly llvm-mos, possibly a custom codegen backend | Banked 24-bit addresses, 8- and 16-bit modes |

## Nintendo DS

The DS's ARM7 is the GBA's CPU, so much of the GBA SDK's low-level work carries over, and so does the mrustc path through `arm-none-eabi-gcc`. Nothing about the compilers is new. The new work is DS hardware: the two CPUs and how they share memory and messages, the 3D engine and the touch screen. BlocksDS (a C SDK) and melonDS (an emulator) are the references.

## Nintendo 3DS and PlayStation Vita

Both have targets built into Rust, and unlike every other console here, both targets come with `std`: `armv6k-nintendo-3ds` and `armv7-sony-vita-newlibeabihf`. They are Tier 3, so `std` is built from source with `-Zbuild-std`, the same mechanism the GBA uses for `core`. Games for them are ordinary Rust programs with threads, files, collections and formatting, running on top of the console's operating system rather than on bare hardware.

That changes what the SDK is. There is no startup code, linker script or register map to write, since the operating system and its C libraries own the hardware. The SDK is an API over those libraries for graphics, input, audio and saves, designed to feel like the other SDKs in the collection.

- **3DS**: the target links against [libctru](https://github.com/devkitPro/libctru), the C library for Horizon OS, and uses devkitPro's toolchain. [ctru-rs](https://github.com/rust3ds/ctru-rs) already wraps libctru in Rust, so the first step is deciding whether to build on it or beside it. It comes after the DS because the 3DS keeps the DS's two-screen, touch-screen shape, so the DS SDK's API ideas carry over even though the code doesn't.
- **Vita**: the target uses [VitaSDK](https://github.com/vitasdk)'s toolchain and libraries, and [cargo-vita](https://github.com/vita-rust/cargo-vita) already packages Rust programs into installable `.vpk` files. The same build-on-it-or-beside-it question applies.

mrustc isn't planned for either. Its value on the GBA is GCC's code generation for a small target, and these consoles already have mature C toolchains, plenty of memory, and a `std` that mrustc's output would have to match.

## Nintendo Switch

The Switch is a maybe, and its place in the order depends on how well it can be tested when work would start.

The compiler side costs nothing: Rust has `aarch64-nintendo-switch-freestanding`, a Tier 3 ARM64 target that LLVM handles well. Unlike the 3DS and Vita targets it comes without `std`, so the SDK stays `no_std` and binds [libnx](https://github.com/switchbrew/libnx), the established C library for Switch homebrew, itself. Like the 3DS and Vita SDKs, it would be an API over the operating system rather than over bare hardware.

Like the 3DS and Vita, the Switch only runs homebrew under custom firmware ([Atmosphère](https://github.com/Atmosphere-NX/Atmosphere)), and getting there is the player's risk, not the SDK's: software can't brick a console by running as an app. The route in is narrower than on the other two, though. Only early Switch units can be modified by software; later ones need a soldered modchip, which is where real hardware damage happens. Running the firmware from the SD card (emuMMC) keeps the console's own storage untouched, which makes a ban for going online the more likely problem. The docs for this SDK would say all of that up front.

The real obstacle is testing. Every other SDK here runs its test ROMs headless in an emulator. Nintendo shut down the main Switch emulators, Yuzu and Ryujinx, in 2024, and forks continue, but none is something to build a test setup on yet, so testing would lean on real hardware far more than for any other console. This plan covers the original Switch family only, since the Switch 2 has no known homebrew route.

## Nintendo 64

There is no ready-made Rust target for the N64, but `mips-mti-none-elf` (32-bit, big-endian MIPS with no operating system) is a close starting point for a custom one, built with `-Zbuild-std` like the GBA. GCC for `mips64-elf` gives mrustc a path too, and with 4 to 8 MB of RAM, mrustc's larger output matters less than on the GBA. libdragon, the established C SDK, is the main reference. The hard part is the RSP: it runs its own microcode, which the SDK either borrows from libdragon or writes in assembly.

## 6502 consoles

LLVM has no 6502 backend, but [llvm-mos](https://github.com/llvm-mos/llvm-mos) is a maintained port of LLVM to the 6502 family, and its SDK already supports the NES (with several mappers), the PC Engine and others. A Rust fork built on it exists, but it has fallen behind current Rust, so using it means pinning an old compiler or keeping the fork up to date ourselves.

The compiler is the easy part here. The NES has 2 KB of RAM and a 1.79 MHz 8-bit CPU, `usize` is 16 bits, `core::fmt` alone can outgrow a small game, and ROM comes in banks the SDK has to switch. An SDK for these consoles would look leaner than the GBA's: panics without messages, a much smaller logger, and careful bank handling.

## Super Nintendo

The SNES is the hardest: its 65816 addresses 24 bits of memory through banks while its registers are 16 bits, and switches its registers between 8 and 16 bits at run time.

llvm-mos already knows the 65816's instructions, but how far its code generation goes is unknown: it may treat the chip as a 6502 with more instructions, without its 16-bit mode or banked addressing. A small spike decides it: compile a function that adds two `u16`s for llvm-mos's 65816 and read the assembly.

If that comes up short, the fallback is our own rustc codegen backend, which plugs into `rustc` the way `rustc_codegen_gcc` does and turns rustc's MIR into 65816 assembly text for an assembler such as ca65. A first version can be simple (every value in a memory slot, no register allocator) and still be correct.

mrustc is ruled out for the SNES: it needs a C compiler for the 65816 that can take its generated C, and none of the existing ones is a good fit.

## Others

- **Sega Genesis**: Rust already has `m68k-unknown-none-elf`, a bare-metal target for its 68000, built through LLVM's younger M68k backend. It needs no compiler work and can come at any point.
- **PlayStation**: Rust has `mipsel-sony-psx`, a ready-made target.

Facts on this page that change with time, like the state of llvm-mos's 65816 support, the Rust fork, ctru-rs and cargo-vita, Switch emulators and the Switch 2's homebrew status, and Rust's list of targets, get checked again before work on a console starts.
