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

//! A controller that lost its state. A Hardware Error (Vol 4, Part E,
//! 7.7.16) says so outright; Command Complete for a Reset (7.3.2) this host
//! did not send says the controller reset itself. Either way every link,
//! buffer count and mask the host set is gone, and nothing it does next
//! means what it meant, so the host stops with `Reset` and the caller
//! brings the controller up again. While this host waits on its own
//! Reset, both are the past being cleared and pass.

use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::hci::event::{parse_event, CommandComplete};
use nonos_bt_wire::hci::event_code::{COMMAND_COMPLETE, HARDWARE_ERROR};
use nonos_bt_wire::hci::opcode::RESET;

impl<T: Transport> Host<T> {
    pub(super) fn lost(&self, event: &[u8]) -> Result<(), HciError> {
        if self.resetting {
            return Ok(());
        }
        let ev = parse_event(event)?;
        let reset = match ev.code {
            HARDWARE_ERROR => true,
            COMMAND_COMPLETE => CommandComplete::parse(ev.params).is_ok_and(|c| c.opcode == RESET),
            _ => false,
        };
        if reset {
            return Err(HciError::Reset);
        }
        Ok(())
    }
}
