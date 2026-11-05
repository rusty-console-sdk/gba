//! The runtime every ROM links: startup, the linker script and the panic
//! handler.
//!
//! The startup code is `crt0.s`, assembled by `just asm` and linked through
//! `gba.ld`. It sets the stacks, copies `.data` into IWRAM, zeroes `.bss`
//! and calls `main`, which a ROM defines with [`entry!`].
//!
//! Panics abort: the handler writes the message to mGBA's log and halts.

#![no_std]

use core::fmt::Write;
use core::panic::PanicInfo;

use gba_sdk_regs::mgba;

/// Defines the ROM's entry point.
///
/// The function takes no arguments and never returns. It runs after the
/// runtime has set up memory, with interrupts masked by `IME`.
///
/// ```ignore
/// gba_sdk_rt::entry!(main);
///
/// fn main() -> ! {
///     loop {}
/// }
/// ```
#[macro_export]
macro_rules! entry {
    ($f:path) => {
        #[export_name = "main"]
        pub extern "C" fn __gba_sdk_main() -> ! {
            let f: fn() -> ! = $f;
            f()
        }
    };
}

/// Levels for mGBA's log, in its own numbering.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum LogLevel {
    Fatal = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
}

/// A line of mGBA's log. Does nothing on hardware, or in emulators without
/// the interface.
pub struct MgbaLog {
    level: LogLevel,
    len: usize,
}

impl MgbaLog {
    /// Starts a line at `level`. Returns `None` when there is no mGBA to
    /// receive it.
    pub fn new(level: LogLevel) -> Option<Self> {
        mgba::ENABLE.write(mgba::ENABLE_CODE);
        if mgba::ENABLE.read() != mgba::ENABLED_CODE {
            return None;
        }
        Some(Self { level, len: 0 })
    }
}

impl Write for MgbaLog {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // One line is at most 255 bytes; longer text is cut.
        for &b in s.as_bytes() {
            if self.len == mgba::STRING.len() - 1 {
                break;
            }
            mgba::STRING.get(self.len).write(b);
            self.len += 1;
        }
        Ok(())
    }
}

impl Drop for MgbaLog {
    fn drop(&mut self) {
        mgba::STRING.get(self.len).write(0);
        mgba::FLAGS.write(mgba::SEND | self.level as u16);
    }
}

/// Writes one line to mGBA's log, like `println!`.
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        if let Some(mut line) = $crate::MgbaLog::new($level) {
            let _ = ::core::fmt::Write::write_fmt(&mut line, format_args!($($arg)*));
        }
    };
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log!(LogLevel::Fatal, "{}", info);
    loop {}
}
