// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's register accessor on the host, with the capsule's signature.
//! A port's registers are host memory the test owns, unless the test has
//! plugged in a model port (`model::plug`): then every read and write goes
//! to the model, which answers as an HBA does, write-one-to-clear bits,
//! self-clearing commands and all.

use core::ptr::{read_volatile, write_volatile};

#[derive(Clone, Copy)]
pub struct Regs {
    base: u64,
}

impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { base }
    }

    /// # Safety
    /// Without a model port, `base + off` names a u32 of memory the caller
    /// owns.
    pub unsafe fn r32(self, off: u32) -> u32 {
        match crate::model::read(off) {
            Some(v) => v,
            // SAFETY: the caller's contract above.
            None => unsafe { read_volatile((self.base + off as u64) as *const u32) },
        }
    }

    /// # Safety
    /// As `r32`.
    pub unsafe fn w32(self, off: u32, value: u32) {
        if !crate::model::write(off, value) {
            // SAFETY: the caller's contract above.
            unsafe { write_volatile((self.base + off as u64) as *mut u32, value) }
        }
    }
}
