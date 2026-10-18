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

//! The link's outbox, emptied as the controller frees buffers.

use super::session::Session;
use nonos_bt_hci::{HciError, Host, Transport};

impl Session {
    /// Send what waits while the controller has buffers; a PDU that does
    /// not fit waits whole for the next turn (`Host::send_pdu`).
    pub(super) fn flush<T: Transport>(&mut self, host: &mut Host<T>) -> Result<(), HciError> {
        // A link that has ended sends nothing more, its encryption maybe gone.
        if self.end.is_some() {
            return Ok(());
        }
        while let Some(frame) = self.outbox.front() {
            match host.send_pdu(self.handle, frame) {
                Ok(()) => {
                    self.outbox.pop_front();
                    self.stuck_since = None;
                }
                Err(HciError::NoCredit) => {
                    self.short = true;
                    break;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}
