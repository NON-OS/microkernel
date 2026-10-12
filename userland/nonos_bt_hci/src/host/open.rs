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
//! Opening a link: LE Create Connection, then LE Connection Complete
//! (Vol 4, Part E, 7.7.65.1), with a cancel when none comes in time.

use super::connect::{create_connection_params_as, Connect};
use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::hci::event::parse_event;
use nonos_bt_wire::hci::event_code::{LE_CONNECTION_COMPLETE, LE_META};
use nonos_bt_wire::hci::le_meta::{split, LeConnectionComplete};
use nonos_bt_wire::hci::opcode as op;

/// The status a cancel gets when no connection attempt is pending
/// (Vol 4, Part E, 7.8.13; Vol 1, Part F, 2).
const COMMAND_DISALLOWED: u8 = 0x0c;

impl<T: Transport> Host<T> {
    /// Connect to `c.peer` as central. If no LE Connection Complete comes
    /// within the poll budget, LE Create Connection Cancel (Vol 4, Part E,
    /// 7.8.13) stops the attempt and the controller ends it with LE
    /// Connection Complete, status Unknown Connection Identifier; that
    /// event is taken here and the answer is `Timeout`.
    pub fn connect(&mut self, c: &Connect) -> Result<LeConnectionComplete, HciError> {
        let create = op::LE_CREATE_CONNECTION;
        self.command_pending(create, &create_connection_params_as(c, self.own)?)?;
        if let Some(done) = self.wait_for(connection_complete)? {
            return match done.status {
                0 => Ok(done),
                s => Err(HciError::Status(create, s)),
            };
        }
        // Disallowed means the attempt ended before the cancel came: the
        // link is up and its LE Connection Complete is on the way.
        match self.command(op::LE_CREATE_CONNECTION_CANCEL, &[]) {
            Ok(_) | Err(HciError::Status(_, COMMAND_DISALLOWED)) => {}
            Err(e) => return Err(e),
        }
        match self.wait_for(connection_complete)? {
            Some(done) if done.status == 0 => Ok(done),
            _ => Err(HciError::Timeout(create)),
        }
    }
}

fn connection_complete(event: &[u8]) -> Result<Option<LeConnectionComplete>, HciError> {
    let ev = parse_event(event)?;
    if ev.code != LE_META {
        return Ok(None);
    }
    match split(ev.params)? {
        (LE_CONNECTION_COMPLETE, sub) => Ok(Some(LeConnectionComplete::parse(sub)?)),
        _ => Ok(None),
    }
}
