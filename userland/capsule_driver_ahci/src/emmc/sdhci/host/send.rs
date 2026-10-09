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

//! One command sent and waited for, through its data or busy phase.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::regs::{
    ERR_CMD_CRC, ERR_CMD_END_BIT, ERR_CMD_INDEX, ERR_CMD_TIMEOUT, INT_CMD_COMPLETE, RESPONSE,
};
use super::cmd::Cmd;
use super::engine::Host;
use super::limits::CMD_MS;
use super::wait::Waited;

/// The Error Interrupt Status bits that belong to the command phase.
const CMD_ERRORS: u16 = ERR_CMD_TIMEOUT | ERR_CMD_CRC | ERR_CMD_END_BIT | ERR_CMD_INDEX;

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    /// Send one command and wait for it, and for its data or busy phase,
    /// to end. Returns the response registers: the R1 or R3 word in [0], or
    /// for R2 the four words holding response bits 127:8 in their 119:0.
    pub fn send(&mut self, cmd: &Cmd) -> EmmcResult<[u32; 4]> {
        let uses_dat = cmd.data.is_some() || cmd.resp.busy();
        self.issue(cmd, uses_dat)?;
        if let Err(w) = self.wait(INT_CMD_COMPLETE, CMD_MS) {
            self.clear_lines();
            return Err(match w {
                Waited::Timeout => EmmcError::NoCompletion(cmd.index),
                Waited::Error(e) if e == ERR_CMD_TIMEOUT => EmmcError::CmdTimeout(cmd.index),
                // The command was answered, and the data phase already
                // failed by the time it was looked at.
                Waited::Error(e) if e & CMD_ERRORS == 0 => {
                    EmmcError::DataError { cmd: cmd.index, err: e }
                }
                Waited::Error(e) => EmmcError::CmdError { cmd: cmd.index, err: e },
            });
        }
        let resp = if cmd.resp.long() {
            [
                self.io.r32(RESPONSE),
                self.io.r32(RESPONSE + 4),
                self.io.r32(RESPONSE + 8),
                self.io.r32(RESPONSE + 12),
            ]
        } else {
            [self.io.r32(RESPONSE), 0, 0, 0]
        };
        if uses_dat {
            self.transfer(cmd)?;
        }
        Ok(resp)
    }
}
