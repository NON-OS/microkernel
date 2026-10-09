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

use super::ocp::{mac_ocp_modify, mac_ocp_modify_all as modify_all, mac_ocp_read, mac_ocp_write};
use super::ocp_8125::{by_version, BEFORE_FORMAT, FORMAT, TAIL};
use super::{disable_rxdvgate, hold_ms, wait_for};
use crate::chip::MacVersion;
use crate::log::Line;
use crate::regs::Regs;

fn and_not8(regs: &Regs, offset: usize, bits: u8) {
    // SAFETY: Config1 and Config3 lie inside every mapped window.
    unsafe { regs.w8(offset, regs.r8(offset) & !bits) };
}

/*
 * Linux rtl_hw_start_8125 and rtl_hw_start_8125_common, VER_61 to VER_66,
 * after the per-revision ePHY table (not repeated: firmware's PCIe tuning).
 * Registers past 0x100 (0x382, 0x1880, RSS_CTRL 0x4500, Q_NUM_CTRL 0x4800,
 * the coalescing block at 0xA00) are inside the window chip::detect asks an
 * 8125 for. The extended tally counter (MAC OCP 0xEA84) is not turned on:
 * nothing reads it. Ends with the RX data gate released; until then no
 * frame is received.
 */
pub fn start_8125(regs: &Regs, ver: MacVersion) {
    // SAFETY (each block): every offset written lies below 0x4804, inside
    // the window chip::detect insists on for an 8125.
    unsafe {
        regs.w8(0x34, 0); // INT_CFG0_8125
        let end = if ver.0 == 63 { 0xA80 } else { 0xB00 };
        (0xA00..end).step_by(4).for_each(|off| regs.w32(off, 0));
        if ver.0 == 63 {
            regs.w16(0x7A, 0); // INT_CFG1_8125
        }
    }
    and_not8(regs, 0x54, 1 << 1); // Config3 Rdy_to_L23
                                  // SAFETY: as above.
    unsafe {
        regs.w16(0x382, 0x221B);
        regs.w32(0x4500, 0);
        regs.w16(0x4800, 0);
    }
    modify_all(regs, BEFORE_FORMAT);
    and_not8(regs, 0x52, 0x10); // Config1
    mac_ocp_write(regs, 0xC140, 0xFFFF);
    mac_ocp_write(regs, 0xC142, 0xFFFF);
    modify_all(regs, FORMAT);
    modify_all(regs, &by_version(ver.0));
    modify_all(regs, TAIL);
    mac_ocp_modify(regs, 0xEB54, 0, 1);
    hold_ms(1);
    mac_ocp_modify(regs, 0xEB54, 1, 0);
    // SAFETY: as above.
    unsafe { regs.w16(0x1880, regs.r16(0x1880) & !0x0030) };
    mac_ocp_write(regs, 0xE098, 0xC302);
    if !wait_for(10, false, || mac_ocp_read(regs, 0xE00E) & (1 << 13) != 0) {
        Line::new("rtl8169: MAC OCP 0xE00E bit 13 still set after 10 ms, going on").send();
    }
    disable_rxdvgate(regs);
}
