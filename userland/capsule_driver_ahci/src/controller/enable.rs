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

use super::restore::restore_writes;
use crate::clock::wait_until;
use crate::constants::regs::{
    BOHC_BB, BOHC_BOS, BOHC_OOS, CAP2_BOH, GHC_AE, GHC_HR, HBA_BOHC, HBA_CAP, HBA_CAP2, HBA_GHC,
    HBA_IS, HBA_PI,
};
use crate::constants::timing::{HANDOFF_BOS_MS, HANDOFF_BUSY_MS, HBA_RESET_MS};
use crate::error::{AhciError, AhciResult};
use crate::log::Line;
use crate::regs::Regs;

/// Take the HBA from the firmware and reset it into AHCI mode. `Timeout` when
/// GHC.HR is still set after a second: the HBA did not reset and cannot be
/// trusted with DMA.
pub fn enable_ahci(regs: Regs) -> AhciResult<()> {
    take_from_bios(regs);
    unsafe {
        let saved_cap = regs.r32(HBA_CAP);
        let saved_pi = regs.r32(HBA_PI);
        // AHCI 1.3.1 section 10.4.3. Enter AHCI mode, then reset the HBA so we
        // start from a known state rather than trusting whatever the firmware
        // left mid-configuration. HR self-clears when the reset completes and
        // also clears AE, so AHCI mode is re-entered afterwards.
        regs.w32(HBA_GHC, regs.r32(HBA_GHC) | GHC_AE);
        regs.w32(HBA_GHC, regs.r32(HBA_GHC) | GHC_HR);
        if !wait_until(HBA_RESET_MS, || regs.r32(HBA_GHC) & GHC_HR == 0) {
            Line::new().text(b"GHC.HR still set after the reset; controller left alone").send();
            return Err(AhciError::Timeout);
        }
        regs.w32(HBA_GHC, regs.r32(HBA_GHC) | GHC_AE);
        let (cap, pi) = restore_writes(saved_cap, saved_pi, regs.r32(HBA_CAP), regs.r32(HBA_PI));
        if let Some(cap) = cap {
            regs.w32(HBA_CAP, cap);
        }
        if let Some(pi) = pi {
            regs.w32(HBA_PI, pi);
        }
        if cap.is_some() || pi.is_some() {
            Line::new()
                .text(b"reset changed CAP/PI; wrote back CAP ")
                .hex(saved_cap)
                .text(b" PI ")
                .hex(saved_pi)
                .text(b", now CAP ")
                .hex(regs.r32(HBA_CAP))
                .text(b" PI ")
                .hex(regs.r32(HBA_PI))
                .send();
        }
        regs.w32(HBA_IS, 0xffff_ffff);
    }
    Ok(())
}

/// BIOS/OS handoff (AHCI 1.3.1, 10.6.3). An HBA with CAP2.BOH may still be
/// owned by the firmware, which can be mid-command on it (BOHC.BB). Ask for
/// it (OOS), give the firmware 25 ms to let go (BOS clear), and when it says
/// it is busy, up to 2 s more. A firmware that never lets go is logged and
/// the reset goes ahead: the HBA reset ends whatever it was doing.
fn take_from_bios(regs: Regs) {
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
