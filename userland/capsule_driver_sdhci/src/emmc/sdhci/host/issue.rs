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

//! Putting one command on the bus once the lines it needs are free.

use super::super::super::env::{Clock, Deadline, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::cmd::{command_word, transfer_mode};
use super::super::regs::{
    ARGUMENT, COMMAND, INT_STATUS, PRESENT_STATE, PS_CMD_INHIBIT, PS_DAT_INHIBIT, TRANSFER_MODE,
};
use super::cmd::Cmd;
use super::engine::Host;
use super::limits::INHIBIT_MS;

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    /// Wait for CMD inhibit, and DAT inhibit when the command uses the
    /// data lines and is not an abort, then program the data phase and
    /// write the Command register, which starts the command.
    pub(super) fn issue(&mut self, cmd: &Cmd, uses_dat: bool) -> EmmcResult<()> {
        let mut mask = PS_CMD_INHIBIT;
        if uses_dat && !cmd.abort {
            mask |= PS_DAT_INHIBIT;
        }
        let d = Deadline::after(&self.clock, INHIBIT_MS);
        while self.io.r32(PRESENT_STATE) & mask != 0 {
            if d.passed(&self.clock) {
                self.clear_lines();
                return Err(EmmcError::Inhibit(cmd.index));
            }
            self.clock.relax();
        }
        self.io.w32(INT_STATUS, u32::MAX);
        let mut mode = 0u16;
        if let Some(data) = cmd.data.as_ref() {
            self.program_data(data)?;
            mode = transfer_mode(data.read, data.multi, data.auto12);
        }
        self.io.w32(ARGUMENT, cmd.arg);
        self.io.w16(TRANSFER_MODE, mode);
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        self.io.w16(COMMAND, command_word(cmd.index, cmd.resp, cmd.data.is_some(), cmd.abort));
        Ok(())
    }
}
