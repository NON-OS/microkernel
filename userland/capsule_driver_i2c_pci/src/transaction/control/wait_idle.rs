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

use crate::constants::{IC_STATUS, IC_STATUS_MST_ACTIVITY, IDLE_TIMEOUT_MS};
use crate::regs::Regs;
use crate::transaction::TransferError;

pub fn wait_idle(regs: Regs) -> Result<(), TransferError> {
    let deadline = Deadline::after_ms(IDLE_TIMEOUT_MS);
    loop {
        if regs.read32(IC_STATUS) & IC_STATUS_MST_ACTIVITY == 0 {
            return Ok(());
        }
        if deadline.expired() {
            return Err(TransferError::Busy);
        }
        core::hint::spin_loop();
    }
}
