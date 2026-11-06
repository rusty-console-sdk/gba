# Roadmap

Finished items stay in their section with a check mark, so each section shows how far along it is. The milestones are in order: each one ends with ROMs that pass on every backend that is still in play.

## Milestone 0: the spike

- [x] Startup code, linker script and a ROM that boots in mGBA.
- [x] Mode 3, the buttons, `wait_for_vblank`.
- [x] `log!` and panics through mGBA's debug registers.
- [x] `just` recipes for check, build and test, headless mGBA, remote builds.
- [x] This site.
- [ ] mrustc: build `core` 1.90 for a custom target and boot the hello ROM through `arm-none-eabi-gcc`.
- [ ] Record ROM size and frame cost for both backends, and decide go or no-go on mrustc.

## Milestone 1: foundations

- [ ] A test runner over libmgba, test ROMs under `tests/`, every test on both backends.
- [ ] Interrupts: the trampoline in `rt`, `VBlankIntrWait`, `wait_for_vblank` that sleeps.
- [ ] `log!` that is safe to interrupt: mGBA's string buffer is shared, so a handler that logs would corrupt a line in progress.
- [ ] Panics on screen as well as in the log.
- [ ] Memory functions in `rt` as assembly, shared by both backends.
- [ ] A test that every `.iwram` function really is in IWRAM.
- [ ] CI: `just gba::check` and `just gba::test` on every push.

## Milestone 2: graphics

- [ ] Tiled backgrounds, sprites, palettes.
- [ ] Windows and blending.
- [ ] DMA, including a fill for `Mode3::clear`.
- [ ] The asset converter for images.
- [ ] Frame comparisons against goldens.

## Milestone 3: game loop essentials

- [ ] Button edges: pressed and released this frame.
- [ ] Timers.
- [ ] Fixed-point math.
- [ ] An optional allocator over EWRAM.
- [ ] Saves: SRAM, then flash and EEPROM.

## Milestone 4: sound

- [ ] Direct Sound with a mixer in IWRAM.
- [ ] The PSG channels.

## Milestone 5: a real game

- [ ] A small complete game in `gba/examples/`, built on both backends.
- [ ] `cargo gba-sdk`, so a game outside this repository builds the same way.
- [ ] Publish `gba-sdk` on crates.io.
- [ ] A run on hardware with a flash cart for everything timing-sensitive.
