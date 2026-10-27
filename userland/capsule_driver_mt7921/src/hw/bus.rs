/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The chip's register window, BAR0, as 32-bit accesses at byte offsets.
//! The driver reaches the chip only through `Bus`, so the host proofs put
//! a modelled chip behind the same calls the capsule makes.

use nonos_libc::{mmio_read32, mmio_write32};

pub trait Bus {
    fn read32(&self, off: u32) -> u32;
    fn write32(&self, off: u32, val: u32);
}

/// BAR0 as the broker mapped it for this claim.
pub struct Mmio {
    base: usize,
    len: usize,
}

impl Mmio {
    /// # Safety
    /// `base` is the start of `len` bytes of this claim's BAR0 mapping, kept
    /// for as long as this lives.
    pub const unsafe fn new(base: u64, len: u64) -> Self {
        Self { base: base as usize, len: len as usize }
    }

    fn inside(&self, off: u32) -> Option<usize> {
        let off = off as usize;
        (off.is_multiple_of(4) && off.checked_add(4)? <= self.len).then_some(self.base + off)
    }
}

impl Bus for Mmio {
    /// An offset outside the window reads as all ones, as a gone card does.
    fn read32(&self, off: u32) -> u32 {
        match self.inside(off) {
            // SAFETY: `new`'s caller vouched for the window, and `inside`
            // put the aligned word within it.
            Some(addr) => unsafe { mmio_read32(addr) },
            None => u32::MAX,
        }
    }

    /// A write outside the window is dropped.
    fn write32(&self, off: u32, val: u32) {
        if let Some(addr) = self.inside(off) {
            // SAFETY: as `read32`.
            unsafe { mmio_write32(addr, val) }
        }
    }
}
