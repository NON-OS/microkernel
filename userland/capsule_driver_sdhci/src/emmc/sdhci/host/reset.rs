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

//! Software reset of the whole host or of its CMD and DAT lines.

use super::super::super::env::{Clock, Deadline, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::regs::{INT_STATUS, RESET_CMD, RESET_DATA, SOFTWARE_RESET};
use super::engine::Host;
use super::limits::RESET_MS;

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    /// Set Software Reset bits and wait for the host to clear them.
    pub fn reset(&self, mask: u8) -> EmmcResult<()> {
        self.io.w8(SOFTWARE_RESET, mask);
        let d = Deadline::after(&self.clock, RESET_MS);
        loop {
            if self.io.r8(SOFTWARE_RESET) & mask == 0 {
                return Ok(());
            }
            if d.passed(&self.clock) {
                return Err(EmmcError::HostReset);
            }
            self.clock.relax();
        }
    }

    /// After a failed command: reset the CMD line, then the DAT line (as
    /// sdhci_request_done does), and clear every status.
    pub(super) fn clear_lines(&self) {
        let _ = self.reset(RESET_CMD);
        let _ = self.reset(RESET_DATA);
        self.io.w32(INT_STATUS, u32::MAX);
    }
}
