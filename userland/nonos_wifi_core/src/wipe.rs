/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Clearing secrets so the compiler cannot drop the writes: a plain fill of
//! memory never read again may be removed as a dead store; these may not.

use core::sync::atomic::{compiler_fence, Ordering};

/// Zero `buf` with volatile writes, then fence.
pub fn wipe(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        // SAFETY: `b` is a valid, aligned, exclusive reference into `buf`.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    compiler_fence(Ordering::SeqCst);
}

/// Overwrite `v`, a secret kept as a value rather than bytes, with `zero` by
/// a volatile write, then fence.
pub fn wipe_value<T: Copy>(v: &mut T, zero: T) {
    // SAFETY: `v` is a valid, aligned, exclusive reference to a `T`.
    unsafe { core::ptr::write_volatile(v, zero) };
    compiler_fence(Ordering::SeqCst);
}
