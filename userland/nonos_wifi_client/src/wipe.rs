/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Clearing secret bytes so the compiler cannot drop the writes.

use core::sync::atomic::{compiler_fence, Ordering};

/// Zero `buf` with volatile writes. A plain fill of a buffer that is never read
/// again may be removed as a dead store; these may not.
pub fn wipe(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        /*
         * SAFETY: `b` is a valid, aligned, exclusive reference into `buf`.
         */
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    compiler_fence(Ordering::SeqCst);
}
