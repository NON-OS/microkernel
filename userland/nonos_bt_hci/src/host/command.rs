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

//! Sending a command and waiting for its answer. Every command is answered
//! by Command Complete or Command Status, and both say how many commands
//! the controller now takes (Vol 4, Part E, 4.4); a command is sent only
//! while that count is above zero. Other events that come meanwhile are
//! kept for the caller.

use alloc::vec::Vec;

use super::answer::{answer, Answer};
use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::hci::command::command;
use nonos_bt_wire::hci::opcode::Opcode;

impl<T: Transport> Host<T> {
    /// A command answered by Command Complete: its return parameters after
    /// the status octet, which must be zero.
    pub fn command(&mut self, op: Opcode, params: &[u8]) -> Result<Vec<u8>, HciError> {
        match self.exchange(op, params)? {
            Answer::Complete(ret) => match ret.split_first() {
                Some((0, rest)) => Ok(rest.to_vec()),
                Some((&s, _)) => Err(HciError::Status(op, s)),
                None => Err(HciError::Wire(nonos_bt_wire::WireError::Short)),
            },
            // Only taken (or refused): not a command this host can finish.
            Answer::Status(s) => Err(HciError::Status(op, s)),
        }
    }

    pub(super) fn exchange(&mut self, op: Opcode, params: &[u8]) -> Result<Answer, HciError> {
        let packet = command(op, params)?;
        let mut sent = false;
        for _ in 0..self.budget {
            if !sent && self.credits > 0 {
                self.t.send_command(&packet)?;
                (sent, self.credits) = (true, self.credits - 1);
            }
            let Some(event) = self.t.poll_event()? else {
                self.t.pause();
                continue;
            };
            match answer(&event, sent.then_some(op))? {
                None => self.keep(event)?,
                Some((credits, a)) => {
                    self.credits = credits;
                    if let Some(a) = a {
                        return Ok(a);
                    }
                }
            }
        }
        Err(HciError::Timeout(op))
    }
}
