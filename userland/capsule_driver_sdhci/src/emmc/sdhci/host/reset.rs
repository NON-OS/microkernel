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
use super::super::super::text::Line;
use super::super::regs::{ENGINE_STATUS, INT_STATUS, PRESENT_STATE, RESET_CMD, RESET_DATA};
use super::super::regs::{PS_CMD_INHIBIT, PS_DAT_ACTIVE, PS_DAT_INHIBIT, SOFTWARE_RESET};
use super::super::regs::{PS_READ_ACTIVE, PS_WRITE_ACTIVE};
use super::engine::Host;
use super::limits::RESET_MS;

/// Present State while a command, the data lines or a transfer is active.
const BUSY: u32 =
    PS_CMD_INHIBIT | PS_DAT_INHIBIT | PS_DAT_ACTIVE | PS_WRITE_ACTIVE | PS_READ_ACTIVE;

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

    /// After Software Reset For All read clear, the host's word that DMA has
    /// ended (SDHCI 3.00, 2.2.16): Present State must agree before anything
    /// is mapped for the host or it is given Bus Master. `HostBusy` when a
    /// command, the data lines or a transfer still show active.
    pub(super) fn idle_after_reset(&self) -> EmmcResult<()> {
        if self.io.r32(PRESENT_STATE) & BUSY != 0 {
            self.say(Line::new().s(b"host still busy after its reset"));
            return Err(EmmcError::HostBusy);
        }
        Ok(())
    }

    /// After a failed command: the CMD line's state is what failed first,
    /// so it is reset first, then the DAT line in case a data phase began;
    /// then the engine's statuses are cleared.
    pub(super) fn clear_lines(&self) {
        let _ = self.reset(RESET_CMD);
        let _ = self.reset(RESET_DATA);
        self.io.w32(INT_STATUS, ENGINE_STATUS);
    }
}
