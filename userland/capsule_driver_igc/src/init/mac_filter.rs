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

//! Receive address and multicast filters, as igc_init_rx_addrs and
//! igc_init_hw_base leave them: the drawn address in RAR0 with AV, every
//! other RAR entry cleared, and the 128-entry MTA zeroed. Firmware or a
//! previous OS can leave old entries, and the factory address in RAR0 is
//! the identifier the drawn one replaces.

use crate::constants::ctrl::RAH_AV;
use crate::constants::regs::{
    MTA_ENTRY_COUNT, RAR_ENTRIES, RAR_STRIDE, REG_MTA_BASE, REG_RAH0, REG_RAL0,
};
use crate::constants::MAC_LEN;
use crate::regs::Regs;

pub fn program(regs: &Regs, mac: &[u8; MAC_LEN]) {
    let low = u32::from_le_bytes([mac[0], mac[1], mac[2], mac[3]]);
    let high = u32::from_le_bytes([mac[4], mac[5], 0, 0]) | RAH_AV;
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0; RAL/RAH
    // 0..16 and MTA 0..128 are 32-bit registers inside it (igc_regs.h).
    unsafe {
        for i in 1..RAR_ENTRIES {
            regs.w32(REG_RAL0 + i * RAR_STRIDE, 0);
            regs.w32(REG_RAH0 + i * RAR_STRIDE, 0);
        }
        // RAL before RAH: the entry only goes live with the AV write.
        regs.w32(REG_RAL0, low);
        regs.w32(REG_RAH0, high);
        for i in 0..MTA_ENTRY_COUNT {
            regs.w32(REG_MTA_BASE + i * 4, 0);
        }
    }
}
