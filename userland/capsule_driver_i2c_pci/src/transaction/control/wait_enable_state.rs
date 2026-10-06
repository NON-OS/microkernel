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

use crate::constants::{ENABLE_TIMEOUT_MS, IC_ENABLE_STATUS};
use crate::regs::Regs;
use crate::transaction::TransferError;

pub fn wait_enable_state(regs: Regs, enabled: bool) -> Result<(), TransferError> {
    let want = if enabled { 1 } else { 0 };
    let deadline = Deadline::after_ms(ENABLE_TIMEOUT_MS);
    loop {
        if regs.read32(IC_ENABLE_STATUS) & 1 == want {
            return Ok(());
        }
        if deadline.expired() {
            return Err(TransferError::Timeout);
        }
        core::hint::spin_loop();
    }
}
