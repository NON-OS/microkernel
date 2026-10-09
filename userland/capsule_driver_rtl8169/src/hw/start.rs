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

use super::regs::{REG_CPLUS_CMD, REG_INTR_MITIGATE};
use super::{cplus_cmd, start_8125, start_8168, start_8168g, start_8169};
use crate::chip::MacVersion;
use crate::constants::regs::{CFG9346_LOCK, CFG9346_UNLOCK, REG_CFG9346, REG_CMD};
use crate::log::Line;
use crate::regs::Regs;

/*
 * Linux rtl_hw_start up to the enable: the config registers unlocked,
 * CPlusCmd, the start for the chip's family, the ring addresses
 * (`program_rings`), the lock put back, and a read of ChipCmd to post it
 * all before TE and RE go on.
 */
pub fn start(regs: &Regs, ver: MacVersion, program_rings: impl FnOnce()) {
    // SAFETY (each block): Cfg9346, CPlusCmd, IntrMitigate and ChipCmd lie
    // below 0x100, inside every mapped window; the driver owns the part.
    unsafe {
        regs.w8(REG_CFG9346, CFG9346_UNLOCK);
        regs.w16(REG_CPLUS_CMD, cplus_cmd(regs.r16(REG_CPLUS_CMD), ver));
    }
    if ver.is_8169() {
        start_8169(regs, ver);
    } else if ver.is_8125() {
        start_8125(regs, ver);
    } else {
        start_8168(regs, ver);
    }
    if ver.is_8168g_up() {
        if let Err(e) = start_8168g(regs, ver) {
            Line::new("rtl8169: 8168g start step failed (").text(e).text("), going on").send();
        }
    }
    if !ver.is_8125() {
        // Linux: "disable interrupt coalescing"; the 8125 moved it.
        // SAFETY: as above.
        unsafe { regs.w16(REG_INTR_MITIGATE, 0) };
    }
    program_rings();
    // SAFETY: as above.
    unsafe {
        regs.w8(REG_CFG9346, CFG9346_LOCK);
        let _ = regs.r8(REG_CMD);
    }
}
