/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Reading and writing a grant. Every access is volatile and checked: one
//! outside the region reads zeros and writes nothing. Descriptor words go
//! as whole aligned words, so a word the chip writes is never seen torn.

use core::ptr::{read_volatile, write_volatile};

use super::grant::Grant;

impl Grant {
    pub fn write(&self, off: usize, src: &[u8]) {
        if self.inside(off, src.len()) {
            for (i, b) in src.iter().enumerate() {
                // SAFETY: `map` or `sub` vouched for [va, va+len), and
                // `inside` put the byte within it.
                unsafe { write_volatile((self.va + off + i) as *mut u8, *b) }
            }
        }
    }

    pub fn read(&self, off: usize, dst: &mut [u8]) {
        let ok = self.inside(off, dst.len());
        for (i, b) in dst.iter_mut().enumerate() {
            // SAFETY: as `write`, only when the whole span is inside.
            *b = if ok { unsafe { read_volatile((self.va + off + i) as *const u8) } } else { 0 };
        }
    }

    pub fn fill(&self, off: usize, n: usize, byte: u8) {
        if self.inside(off, n) {
            for i in 0..n {
                // SAFETY: as `write`.
                unsafe { write_volatile((self.va + off + i) as *mut u8, byte) }
            }
        }
    }

    fn word(&self, off: usize) -> Option<usize> {
        (off.is_multiple_of(4) && self.va.is_multiple_of(4) && self.inside(off, 4))
            .then_some(self.va + off)
    }

    pub fn rd32(&self, off: usize) -> u32 {
        // SAFETY: an aligned word inside the region (`word`).
        self.word(off).map_or(0, |a| u32::from_le(unsafe { read_volatile(a as *const u32) }))
    }

    pub fn wr32(&self, off: usize, v: u32) {
        if let Some(a) = self.word(off) {
            // SAFETY: as `rd32`.
            unsafe { write_volatile(a as *mut u32, v.to_le()) }
        }
    }
}
