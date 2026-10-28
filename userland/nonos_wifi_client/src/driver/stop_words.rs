/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The words `StopWhy` puts on screen for a refused claim.

/// The claim step the driver reports (`setup::claim_note` in the driver).
pub(super) fn claim_step(step: u32) -> Option<&'static [u8]> {
    Some(match step {
        1 => b"claim refused",
        2 => b"memory enable refused",
        3 => b"register map refused",
        _ => return None,
    })
}

/// The broker's errnos (`device_errno.rs`, `mmio/map.rs`) in a word or two.
pub(super) fn errno_word(errno: u32) -> &'static [u8] {
    match errno {
        1 => b"not permitted",
        11 => b"device resetting",
        12 => b"out of memory",
        14 => b"bad address",
        16 => b"busy",
        19 => b"no such device",
        22 => b"invalid request",
        _ => b"error",
    }
}

/// `errno` in decimal, in at most three digits: errnos are small, and one
/// past 999 is shown as 999.
pub(super) fn decimal(errno: u32, out: &mut [u8; 3]) -> &[u8] {
    let e = errno.min(999);
    *out = [b'0' + (e / 100) as u8, b'0' + (e / 10 % 10) as u8, b'0' + (e % 10) as u8];
    let skip = (e < 100) as usize + (e < 10) as usize;
    &out[skip..]
}
