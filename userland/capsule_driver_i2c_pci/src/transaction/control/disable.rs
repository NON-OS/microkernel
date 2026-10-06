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

use crate::constants::{
    ENABLE_TIMEOUT_MS, IC_ENABLE, IC_ENABLE_ABORT, IC_ENABLE_STATUS, IC_INTR_MST_ON_HOLD,
    IC_RAW_INTR_STAT,
};
use crate::regs::Regs;
use crate::transaction::TransferError;

/// Disable the core the way Linux __i2c_dw_disable does. A master holding the
/// bus (its TX FIFO ran dry before a STOP, MST_ON_HOLD) never reports itself
/// disabled, so it is told to ABORT first, which sends the STOP. Then
/// IC_ENABLE is written to zero again on every poll until IC_ENABLE_STATUS
/// follows: the databook allows the disable to be refused while a transfer
/// is in flight, and a single write can be lost that way.
pub fn disable(regs: Regs) -> Result<(), TransferError> {
    if regs.read32(IC_RAW_INTR_STAT) & IC_INTR_MST_ON_HOLD != 0 {
        let enable = regs.read32(IC_ENABLE);
        regs.write32(IC_ENABLE, enable | IC_ENABLE_ABORT);
        let abort = Deadline::after_ms(ENABLE_TIMEOUT_MS);
        while regs.read32(IC_ENABLE) & IC_ENABLE_ABORT != 0 && !abort.expired() {
            core::hint::spin_loop();
        }
    }
    let deadline = Deadline::after_ms(ENABLE_TIMEOUT_MS);
    loop {
        regs.write32(IC_ENABLE, 0);
        if regs.read32(IC_ENABLE_STATUS) & 1 == 0 {
            return Ok(());
        }
        if deadline.expired() {
            return Err(TransferError::Timeout);
        }
        core::hint::spin_loop();
    }
}
