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

use super::established::{link_state, LinkState};
use super::ready::device_ready;
use crate::clock::{pause_ms, wait_until, Deadline};
use crate::constants::regs::{
    PORT_SCTL, PORT_SERR, PORT_SSTS, PORT_TFD, SCTL_DET_COMRESET, SCTL_DET_MASK,
};
use crate::constants::timing::{COMRESET_HOLD_MS, LINK_EMPTY_MS, LINK_TIMEOUT_MS};
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

/// Bring the SATA link up before any command is issued: force a COMRESET so a
/// link the firmware left down (or never spun up) is re-established, wait for
/// the PHY to report communication, clear the errors latched during OOB, then
/// wait up to `ready_ms` for the device to finish its power-on diagnostics: a
/// spinning disk clears BSY only once it is up to speed. Without this,
/// IDENTIFY was issued into a port whose device was still busy, which real
/// drives reject.
///
/// `NoDisk` when the PHY stays quiet for LINK_EMPTY_MS: the port is empty, as
/// Linux's debounce after a hard reset decides it. The engine must be stopped.
pub(crate) fn link_up(regs: Regs, base: u32, ready_ms: u64) -> AhciResult<()> {
    let r = |off: u32| unsafe { regs.r32(base + off) };
    unsafe {
        let sctl = regs.r32(base + PORT_SCTL);
        regs.w32(base + PORT_SCTL, (sctl & !SCTL_DET_MASK) | SCTL_DET_COMRESET);
        pause_ms(COMRESET_HOLD_MS);
        regs.w32(base + PORT_SCTL, regs.r32(base + PORT_SCTL) & !SCTL_DET_MASK);
    }

    let limit = Deadline::after_ms(LINK_TIMEOUT_MS);
    let mut quiet_until: Option<Deadline> = None;
    loop {
        match link_state(r(PORT_SSTS)) {
            LinkState::Up => break,
            LinkState::Negotiating => quiet_until = None,
            LinkState::Quiet => {
                if quiet_until.get_or_insert_with(|| Deadline::after_ms(LINK_EMPTY_MS)).expired() {
                    return Err(AhciError::NoDisk);
                }
            }
        }
        if limit.expired() {
            return Err(AhciError::Timeout);
        }
        core::hint::spin_loop();
    }
    unsafe {
        regs.w32(base + PORT_SERR, regs.r32(base + PORT_SERR));
    }
    if !wait_until(ready_ms, || device_ready(r(PORT_TFD))) {
        return Err(AhciError::Timeout);
    }
    Ok(())
}
