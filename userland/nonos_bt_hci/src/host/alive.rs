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

//! Whether the controller still knows a link. A controller that reset
//! itself without a word, or dropped a link without Disconnection
//! Complete, leaves a quiet link looking up for good; Read RSSI (Vol 4,
//! Part E, 7.5.4) names the link, and a controller that no longer has it
//! answers Unknown Connection Identifier (Vol 1, Part F, 2).

use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::hci::opcode::Opcode;

/// Vol 4, Part E, 7.5.4.
pub const READ_RSSI: Opcode = Opcode::new(0x05, 0x0005);
const UNKNOWN_CONNECTION: u8 = 0x02;

impl<T: Transport> Host<T> {
    /// `false` when the controller says it has no link `handle`.
    pub fn link_known(&mut self, handle: u16) -> Result<bool, HciError> {
        match self.command(READ_RSSI, &handle.to_le_bytes()) {
            Ok(_) => Ok(true),
            Err(HciError::Status(_, UNKNOWN_CONNECTION)) => Ok(false),
            Err(e) => Err(e),
        }
    }
}
