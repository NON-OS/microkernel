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

//! A command answered by Command Status (Vol 4, Part E, 7.7.15): the
//! controller has taken it, and its outcome comes later in an event of its
//! own, such as LE Connection Complete after LE Create Connection.

use super::answer::Answer;
use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::hci::opcode::Opcode;

impl<T: Transport> Host<T> {
    /// `Ok` once the controller has taken the command.
    pub fn command_pending(&mut self, op: Opcode, params: &[u8]) -> Result<(), HciError> {
        match self.exchange(op, params)? {
            Answer::Status(0) => Ok(()),
            Answer::Status(s) => Err(HciError::Status(op, s)),
            Answer::Complete(_) => Err(HciError::Status(op, 0)),
        }
    }
}
