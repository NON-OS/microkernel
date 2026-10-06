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

use crate::log::Line;
use crate::regs::Regs;

/// OCPDR (8168g and later, 8125): MAC OCP access through one dword.
const REG_OCPDR: usize = 0xB0;
const OCPAR_FLAG: u32 = 1 << 31;

/// Linux rtl_ocp_reg_failure: a MAC OCP register is even and below 0x10000.
fn bad(reg: u32) -> bool {
    if reg & 0xFFFF_0001 != 0 {
        Line::new("rtl8169: MAC OCP register ").hex(reg).text(" is not valid, skipped").send();
        return true;
    }
    false
}

/// Linux __r8168_mac_ocp_write. The part takes it at once: Linux does not
/// poll after it, and neither does this.
pub fn mac_ocp_write(regs: &Regs, reg: u32, data: u16) {
    if bad(reg) {
        return;
    }
    // SAFETY: OCPDR (0xB0) lies inside every mapped window.
    unsafe { regs.w32(REG_OCPDR, OCPAR_FLAG | (reg << 15) | data as u32) };
}

/// Linux __r8168_mac_ocp_read: the register number, then the data back.
pub fn mac_ocp_read(regs: &Regs, reg: u32) -> u16 {
    if bad(reg) {
        return 0;
    }
    // SAFETY: as above.
    unsafe {
        regs.w32(REG_OCPDR, reg << 15);
        regs.r32(REG_OCPDR) as u16
    }
}

/// Linux r8168_mac_ocp_modify: clear `clear`, then set `set`.
pub fn mac_ocp_modify(regs: &Regs, reg: u32, clear: u16, set: u16) {
    let val = mac_ocp_read(regs, reg);
    mac_ocp_write(regs, reg, (val & !clear) | set);
}

/// `mac_ocp_modify` over (register, clear, set) rows, in order.
pub fn mac_ocp_modify_all(regs: &Regs, rows: &[(u32, u16, u16)]) {
    rows.iter().for_each(|&(reg, clear, set)| mac_ocp_modify(regs, reg, clear, set));
}
