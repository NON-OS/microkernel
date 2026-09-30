/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/* What setup turned off, held for the rest of the boot. */

use core::sync::atomic::{AtomicU32, Ordering};

static OFF: AtomicU32 = AtomicU32::new(0);

/*
 * Record what setup turned off, of the apps this kernel carries. It only
 * ever adds: nothing turns an app back on before the next boot.
 */
#[cfg(feature = "microkernel-setup-wizard")]
pub(crate) fn choose(off: u8) {
    let off = off as u32 & super::PRESENT;
    OFF.fetch_or(off, Ordering::SeqCst);
    if off != 0 {
        crate::sys::serial::print(b"[INIT] apps turned off at setup, mask ");
        crate::sys::serial::print_hex(off as u64);
        crate::sys::serial::println(b"");
    }
}

pub(crate) fn off() -> u32 {
    OFF.load(Ordering::SeqCst)
}

pub(crate) fn is_off(bit: u32) -> bool {
    off() & bit != 0
}
