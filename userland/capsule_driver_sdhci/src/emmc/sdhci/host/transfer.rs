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

//! The data or busy phase that follows a command's response.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::text::Line;
use super::super::regs::{ADMA_ERROR, AUTO_CMD_ERROR, INT_TRANSFER_COMPLETE};
use super::cmd::Cmd;
use super::engine::Host;
use super::wait::Waited;

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    /// Wait for Transfer Complete. On a failure the ADMA and Auto CMD
    /// error registers are read before the line reset clears them, so the
    /// log names the descriptor or the CMD12 that failed.
    pub(super) fn transfer(&mut self, cmd: &Cmd) -> EmmcResult<()> {
        let Err(w) = self.wait(INT_TRANSFER_COMPLETE, cmd.wait_ms) else {
            return Ok(());
        };
        let adma = self.io.r8(ADMA_ERROR);
        let auto = self.io.r16(AUTO_CMD_ERROR);
        self.clear_lines();
        let e = match w {
            Waited::Timeout => EmmcError::NoCompletion(cmd.index),
            Waited::Error(e) => EmmcError::DataError { cmd: cmd.index, err: e },
        };
        if let EmmcError::DataError { err, .. } = e {
            self.say(
                Line::new()
                    .s(b"CMD")
                    .dec(cmd.index as u64)
                    .s(b" data error ")
                    .hex(err as u64)
                    .s(b" adma ")
                    .hex(adma as u64)
                    .s(b" auto ")
                    .hex(auto as u64),
            );
        }
        Err(e)
    }
}
