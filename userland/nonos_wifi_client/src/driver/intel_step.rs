/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! What an Intel Wi-Fi bring-up step means (the step byte the iwlwifi
//! driver's status reply carries after its stage, `outcome::STEP_*` in the
//! driver), for the panel line that says where the card stopped; the detail
//! word after it says how, and `real-hardware-checklist.md` decodes it.
//! `None` for no step or one this build does not know.

pub fn intel_step_text(step: u8) -> Option<&'static str> {
    Some(match step {
        1 => "register window too small",
        2 => "registers read back all ones",
        3 => "NIC not ready or MAC clock off",
        4 => "no firmware bundled for it",
        5 => "firmware image did not parse",
        6 => "no DMA memory for the radio",
        7 => "firmware never came alive",
        8 => "a firmware setup command failed",
        9 => "firmware hit an error running",
        10 => "bus master did not come on",
        11 => "firmware stopped answering",
        12 => "a sent frame was never answered",
        _ => return None,
    })
}

/// `v` as eight lowercase hex digits.
pub(super) fn hex32(v: u32, out: &mut [u8; 8]) -> &[u8] {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    for (i, b) in out.iter_mut().enumerate() {
        *b = DIGITS[((v >> (28 - 4 * i)) & 0xF) as usize];
    }
    out
}
