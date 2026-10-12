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

//! What one event says to a host waiting on a command: the command count
//! if it carries one, and the answer if it answers the command waited for.
//! Command Complete and Command Status for another opcode, or for opcode
//! 0x0000 (a controller telling it takes commands), give only the count.

use alloc::vec::Vec;

use crate::error::HciError;
use nonos_bt_wire::hci::event::{parse_event, CommandComplete, CommandStatus};
use nonos_bt_wire::hci::event_code::{COMMAND_COMPLETE, COMMAND_STATUS};
use nonos_bt_wire::hci::opcode::Opcode;

pub enum Answer {
    /// Command Complete's return parameters, status first.
    Complete(Vec<u8>),
    /// Command Status's status.
    Status(u8),
}

/// `None` for an event that answers no command; else the command count
/// and, if it answers the command waited for, the answer.
pub fn answer(
    event: &[u8],
    waiting: Option<Opcode>,
) -> Result<Option<(u8, Option<Answer>)>, HciError> {
    let ev = parse_event(event)?;
    let (credits, op, a) = match ev.code {
        COMMAND_COMPLETE => {
            let c = CommandComplete::parse(ev.params)?;
            (c.num_packets, c.opcode, Answer::Complete(c.ret.to_vec()))
        }
        COMMAND_STATUS => {
            let s = CommandStatus::parse(ev.params)?;
            (s.num_packets, s.opcode, Answer::Status(s.status))
        }
        _ => return Ok(None),
    };
    Ok(Some((credits, (waiting == Some(op)).then_some(a))))
}
