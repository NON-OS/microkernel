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

//! LE Connection Complete (Vol 4, Part E, 7.7.65.1) while a link is up,
//! which this host never asked for: it connects as central, one link at a
//! time. One that names this link's handle means the controller has given
//! the handle to another link while this one is open, and nothing that
//! comes on it can be told apart, so this link ends. One that names
//! another handle is a link no one here will serve, and it is ended with
//! Disconnect (7.1.6) at once, its Disconnection Complete left to pass.

use super::session::Session;
use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_wire::hci::le_meta::{split, LeConnectionComplete};
use nonos_bt_wire::hci::opcode as op;

const LE_CONNECTION_COMPLETE: u8 = 0x01;
/// Remote User Terminated Connection (Vol 1, Part F, 2).
const USER_ENDED: u8 = 0x13;

impl Session {
    pub(super) fn on_le<T: Transport>(
        &mut self,
        host: &mut Host<T>,
        params: &[u8],
    ) -> Result<(), HciError> {
        let Ok((LE_CONNECTION_COMPLETE, sub)) = split(params) else { return Ok(()) };
        let Ok(link) = LeConnectionComplete::parse(sub) else { return Ok(()) };
        if link.status != 0 {
            return Ok(());
        }
        if link.handle == self.handle {
            self.give_up("the controller gave this link's handle to another");
            return Ok(());
        }
        let [h0, h1] = link.handle.to_le_bytes();
        match host.command_pending(op::DISCONNECT, &[h0, h1, USER_ENDED]) {
            Ok(()) | Err(HciError::Status(..)) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
