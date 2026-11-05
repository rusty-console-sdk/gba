//! Typed, volatile access to the GBA's memory-mapped registers and memory.
//!
//! This crate holds addresses and the two operations the hardware allows on
//! them, nothing more. Policy (what a register means, when to write it)
//! belongs in `gba-sdk`.
//!
//! Every access goes through `read_volatile` and `write_volatile`. Never
//! take a reference to a register: the compiler may read through a reference
//! whenever it likes, and a read of some registers has side effects.

#![no_std]

use core::marker::PhantomData;

/// A single memory-mapped register or memory cell of type `T`.
///
/// `T` is `u16` or `u32` for the register file. `u8` exists for mGBA's
/// debug string buffer; never use it on VRAM, palette RAM or OAM, which
/// mishandle byte writes (VRAM and palette RAM put the byte in both halves
/// of the halfword, OAM ignores it). The constants here are typed so that
/// can't happen through them.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Reg<T> {
    addr: usize,
    _marker: PhantomData<*mut T>,
}

impl<T: Copy> Reg<T> {
    /// Makes a register at `addr`.
    ///
    /// # Safety
    ///
    /// `addr` must be the address of a readable and writable `T` on the GBA,
    /// aligned for `T`.
    pub const unsafe fn new(addr: usize) -> Self {
        Self {
            addr,
            _marker: PhantomData,
        }
    }

    /// The register's address.
    pub const fn addr(self) -> usize {
        self.addr
    }

    /// Reads the register.
    #[inline]
    pub fn read(self) -> T {
        // SAFETY: `new` requires a readable, aligned address.
        unsafe { (self.addr as *const T).read_volatile() }
    }

    /// Writes the register.
    #[inline]
    pub fn write(self, value: T) {
        // SAFETY: `new` requires a writable, aligned address.
        unsafe { (self.addr as *mut T).write_volatile(value) }
    }
}

/// A contiguous block of `N` cells of type `T`, such as a frame buffer.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Block<T, const N: usize> {
    base: usize,
    _marker: PhantomData<*mut T>,
}

impl<T: Copy, const N: usize> Block<T, N> {
    /// Makes a block starting at `base`.
    ///
    /// # Safety
    ///
    /// `base..base + N * size_of::<T>()` must be readable and writable
    /// memory on the GBA, aligned for `T`.
    pub const unsafe fn new(base: usize) -> Self {
        Self {
            base,
            _marker: PhantomData,
        }
    }

    /// The number of cells.
    pub const fn len(self) -> usize {
        N
    }

    /// Whether the block is empty. It never is, but clippy asks.
    pub const fn is_empty(self) -> bool {
        N == 0
    }

    /// The cell at `index`. Panics when `index` is out of range.
    #[inline]
    pub const fn get(self, index: usize) -> Reg<T> {
        assert!(index < N, "index out of range");
        // SAFETY: `new` covers the whole block, and `index` is in range.
        unsafe { Reg::new(self.base + index * core::mem::size_of::<T>()) }
    }

    /// Writes `value` to every cell.
    pub fn fill(self, value: T) {
        let mut p = self.base as *mut T;
        for _ in 0..N {
            // SAFETY: `new` covers the whole block, and `p` stays within it.
            unsafe {
                p.write_volatile(value);
                p = p.add(1);
            }
        }
    }
}

// SAFETY, for every constant below: the addresses are the GBA's documented
// memory-mapped registers and regions, each aligned for its width, and the
// width is the one the hardware takes.

/// The display control register.
pub const DISPCNT: Reg<u16> = unsafe { Reg::new(0x0400_0000) };
/// The display status register.
pub const DISPSTAT: Reg<u16> = unsafe { Reg::new(0x0400_0004) };
/// The current scanline, 0..=227 (160..=227 is the vertical blank).
pub const VCOUNT: Reg<u16> = unsafe { Reg::new(0x0400_0006) };

/// Key state. A bit is 0 while its key is held.
pub const KEYINPUT: Reg<u16> = unsafe { Reg::new(0x0400_0130) };

/// The master interrupt enable.
pub const IME: Reg<u16> = unsafe { Reg::new(0x0400_0208) };

/// Width of the screen in pixels.
pub const SCREEN_WIDTH: usize = 240;
/// Height of the screen in pixels.
pub const SCREEN_HEIGHT: usize = 160;

/// VRAM as the mode 3 frame buffer: one 15-bit color per pixel.
pub const MODE3_FRAME: Block<u16, { SCREEN_WIDTH * SCREEN_HEIGHT }> =
    unsafe { Block::new(0x0600_0000) };
/// The same frame buffer as words, two pixels each, for faster fills.
pub const MODE3_FRAME_WORDS: Block<u32, { SCREEN_WIDTH * SCREEN_HEIGHT / 2 }> =
    unsafe { Block::new(0x0600_0000) };

/// mGBA's debug interface. These addresses do nothing on hardware.
pub mod mgba {
    use super::{Block, Reg};

    /// The message buffer, 256 bytes. Not hardware memory, so bytes are fine.
    pub const STRING: Block<u8, 256> = unsafe { Block::new(0x04FF_F600) };
    /// Write `SEND | level` to send the buffer as a log line.
    pub const FLAGS: Reg<u16> = unsafe { Reg::new(0x04FF_F700) };
    /// Write `ENABLE_CODE` to turn logging on; reads `ENABLED_CODE` while on.
    pub const ENABLE: Reg<u16> = unsafe { Reg::new(0x04FF_F780) };

    pub const SEND: u16 = 0x100;
    pub const ENABLE_CODE: u16 = 0xC0DE;
    pub const ENABLED_CODE: u16 = 0x1DEA;
}
