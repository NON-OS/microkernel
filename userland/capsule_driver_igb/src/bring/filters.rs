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
//! Receive address and multicast filters: the drawn address in entry 0 with
//! Address Valid, every other entry the MAC has cleared, and the 128-entry
//! multicast table zeroed. Firmware or a previous OS can leave old entries,
//! and the factory address in entry 0 is the identifier the drawn one
//! replaces.
//!
//! The pool bits in each high word stay clear: they select a VMDq pool, and
//! with VMDq off (as reset leaves it) none is read.

use crate::facts::mac::RAH_AV;
use crate::facts::parts::Part;
use crate::facts::regs::{ral, MTA_ENTRY_COUNT, REG_MTA_BASE};
use crate::regs::Regs;
use crate::sizes::MAC_LEN;

pub fn program(regs: &Regs, part: Part, mac: &[u8; MAC_LEN]) {
    let low = u32::from_le_bytes([mac[0], mac[1], mac[2], mac[3]]);
    let high = u32::from_le_bytes([mac[4], mac[5], 0, 0]) | RAH_AV;
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0; RAL/RAH
    // below the part's rar_entries and MTA 0..128 are 32-bit registers
    // inside it (facts::regs).
    unsafe {
        for i in 1..part.rar_entries() {
            regs.w32(ral(i), 0);
            regs.w32(ral(i) + 4, 0);
        }
        // RAL before RAH: the entry only goes live with the AV write.
        regs.w32(ral(0), low);
        regs.w32(ral(0) + 4, high);
        for i in 0..MTA_ENTRY_COUNT {
            regs.w32(REG_MTA_BASE + i * 4, 0);
        }
    }
}
