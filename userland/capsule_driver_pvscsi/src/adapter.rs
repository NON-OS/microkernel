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

//! Resetting the adapter and handing it the rings. Run at bring-up, and
//! again to bring the adapter back after a request it never completed.

use core::ptr::write_bytes;

use crate::constants::layout::{CMP_LOG2, CMP_RING_OFF, PAGE, REQ_LOG2, REQ_RING_OFF, STATE_OFF};
use crate::constants::regs::{
    CMD_ADAPTER_RESET, CMD_SETUP_RINGS, COMMAND_OK, REG_COMMAND, REG_COMMAND_DATA,
    REG_COMMAND_STATUS, REG_INTR_MASK, RS_CMP_LOG2, RS_REQ_LOG2,
};
use crate::regs::Regs;

/// The ring page lists PVSCSICmdDescSetupRings carries.
const RING_PPNS: usize = 32;

/// Reset, every interrupt masked (the driver polls), the rings state and
/// both rings zeroed, then SETUP_RINGS with one page each.
pub fn start<R: Regs>(r: &R, va: u64, dev: u64) -> Result<(), &'static str> {
    r.w32(REG_COMMAND, CMD_ADAPTER_RESET);
    r.w32(REG_INTR_MASK, 0);
    // SAFETY: the first three pages of the grant, mapped for the driver's
    // life; the adapter was just reset and holds no ring.
    unsafe { write_bytes((va as usize + STATE_OFF) as *mut u8, 0, 3 * PAGE) };
    let ppn = |off: usize| (dev + off as u64) >> 12;
    let mut words = [0u32; 4 + 2 * 2 * RING_PPNS];
    words[0] = 1;
    words[1] = 1;
    let state = ppn(STATE_OFF);
    words[2] = state as u32;
    words[3] = (state >> 32) as u32;
    let req = ppn(REQ_RING_OFF);
    words[4] = req as u32;
    words[5] = (req >> 32) as u32;
    let cmp = ppn(CMP_RING_OFF);
    words[4 + 2 * RING_PPNS] = cmp as u32;
    words[5 + 2 * RING_PPNS] = (cmp >> 32) as u32;
    r.w32(REG_COMMAND, CMD_SETUP_RINGS);
    for w in words {
        r.w32(REG_COMMAND_DATA, w);
    }
    if r.r32(REG_COMMAND_STATUS) != COMMAND_OK {
        return Err("rings refused");
    }
    // The adapter fills in the ring sizes; a size it did not take is a
    // layout this driver would overrun.
    let state = |off: usize| {
        // SAFETY: a word of the rings state page, as above.
        unsafe { core::ptr::read_volatile((va as usize + STATE_OFF + off) as *const u32) }
    };
    if state(RS_REQ_LOG2) != REQ_LOG2 || state(RS_CMP_LOG2) != CMP_LOG2 {
        return Err("ring sizes not taken");
    }
    Ok(())
}
