// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The five things the included driver files take from `nonos_libc`.
//!
//! `crypto_random` is the one that matters. The station address is drawn from
//! it, and the driver's promise is that when there is no entropy there is no
//! address programmed at all, rather than the factory one left in place. The
//! test switches it off to hold the driver to that.

use std::sync::atomic::{AtomicBool, Ordering};

static ENTROPY: AtomicBool = AtomicBool::new(true);

/// Whether the next draw succeeds. On by default.
pub fn set_entropy(available: bool) {
    ENTROPY.store(available, Ordering::SeqCst);
}

/// Fills `len` bytes with a fixed pattern when entropy is on, so a test can
/// recognise the drawn address; answers a refusal when it is off.
pub fn crypto_random(ptr: *mut u8, len: usize) -> i64 {
    if !ENTROPY.load(Ordering::SeqCst) {
        return -1;
    }
    for i in 0..len {
        /*
         * SAFETY: the caller hands a buffer of `len` writable bytes, as the
         * real call requires.
         */
        unsafe { *ptr.add(i) = 0xA5 ^ (i as u8) };
    }
    len as i64
}

pub fn mk_device_release(_device_id: u64) -> i64 {
    0
}
pub fn mk_dma_unmap(_grant: u64) -> i64 {
    0
}
pub fn mk_irq_unbind(_grant: u64) -> i64 {
    0
}
pub fn mk_mmio_unmap(_grant: u64) -> i64 {
    0
}
