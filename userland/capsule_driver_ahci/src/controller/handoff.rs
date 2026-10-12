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

use crate::clock::wait_until;
use crate::constants::regs::{BOHC_BB, BOHC_BOS, BOHC_OOS, CAP2_BOH, HBA_BOHC, HBA_CAP2};
use crate::constants::timing::{HANDOFF_BOS_MS, HANDOFF_BUSY_MS};
use crate::log::Line;
use crate::regs::Regs;

/// BIOS/OS handoff (AHCI 1.3.1, 10.6.3). An HBA with CAP2.BOH may still be
/// owned by the firmware, which can be mid-command on it (BOHC.BB). Ask for
/// it (OOS), give the firmware 25 ms to let go (BOS clear), and when it says
/// it is busy, up to 2 s more. A firmware that never lets go is logged and
/// the reset goes ahead: the HBA reset ends whatever it was doing.
pub(super) fn take_from_bios(regs: Regs) {
    // SAFETY: `regs` maps this controller's ABAR, which holds CAP2 and BOHC.
    unsafe {
        if regs.r32(HBA_CAP2) & CAP2_BOH == 0 {
            return;
        }
        let bohc = regs.r32(HBA_BOHC);
        if bohc & BOHC_BOS == 0 {
            return;
        }
        regs.w32(HBA_BOHC, bohc | BOHC_OOS);
        let bios_gone = || regs.r32(HBA_BOHC) & BOHC_BOS == 0;
        if wait_until(HANDOFF_BOS_MS, bios_gone) {
            return;
        }
        let busy = regs.r32(HBA_BOHC) & BOHC_BB != 0;
        if busy && wait_until(HANDOFF_BUSY_MS, || bios_gone() && regs.r32(HBA_BOHC) & BOHC_BB == 0)
        {
            return;
        }
        Line::new().text(b"firmware kept BOHC.BOS: ").hex(regs.r32(HBA_BOHC)).send();
    }
}
