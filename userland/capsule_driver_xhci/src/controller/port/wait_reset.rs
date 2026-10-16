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
use nonos_libc::Deadline;

use crate::constants::{PORTSC_PED, PORTSC_PRC};
use crate::controller::park::{IrqWait, SPIN_BUDGET};
use crate::error::{XhciError, XhciResult};
use crate::regs::op::{controller_failed, portsc_read};

const RESET_TIMEOUT_MS: u64 = 1_000;

/// Wait for the reset to finish with the port enabled. PRC with PED clear
/// is a reset that ended with the device gone or not answering; on silicon
/// that state holds, so it is reported once the deadline passes. A failed
/// controller ends the wait at once: it finishes no reset, and one gone
/// from the bus reads all ones, a reset done.
pub(super) fn wait_reset(op_base: u64, port: u8, irq: &mut IrqWait) -> XhciResult<u32> {
    let deadline = Deadline::after_ms(RESET_TIMEOUT_MS);
    let mut spins = 0u32;
    loop {
        if controller_failed(op_base) {
            return Err(XhciError::ControllerDead);
        }
        let now = portsc_read(op_base, port);
        if (now & PORTSC_PRC) != 0 && (now & PORTSC_PED) != 0 {
            return Ok(now);
        }
        spins = spins.saturating_add(1);
        if spins > SPIN_BUDGET && deadline.expired() {
            return Err(XhciError::PortResetTimeout);
        }
        irq.park(spins);
    }
}
