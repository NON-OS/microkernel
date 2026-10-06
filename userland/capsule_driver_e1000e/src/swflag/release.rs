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

use crate::constants::regs::REG_EXTCNF_CTRL;
use crate::constants::status::EXTCNF_CTRL_SWFLAG;
use crate::regs::Regs;

/// e1000_release_swflag_ich8lan / e1000_put_hw_semaphore_82573. A reset
/// clears the flag on its own; clearing it again is harmless.
pub fn release(regs: &Regs) {
    // SAFETY: `regs` is the broker-mapped BAR0 window; EXTCNF_CTRL is a
    // 4-byte register inside it on every family this driver claims.
    unsafe {
        let v = regs.r32(REG_EXTCNF_CTRL);
        if v & EXTCNF_CTRL_SWFLAG != 0 {
            regs.w32(REG_EXTCNF_CTRL, v & !EXTCNF_CTRL_SWFLAG);
        }
    }
}
