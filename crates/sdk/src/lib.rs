//! Game Boy Advance SDK for Rust.
//!
//! A ROM names its entry point with [`entry!`], draws through [`video`],
//! reads the buttons through [`input`] and syncs to the display with
//! [`video::wait_for_vblank`]. Logging goes to mGBA through [`log!`].

#![no_std]

// Keep the runtime in the link even when a ROM calls nothing in it: it
// holds the panic handler.
extern crate gba_sdk_rt;

pub use gba_sdk_regs as regs;
pub use gba_sdk_rt::{entry, log, LogLevel};

pub mod input;
pub mod video;
