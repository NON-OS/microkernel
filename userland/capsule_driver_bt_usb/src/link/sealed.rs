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

//! A link whose encryption the controller says is gone. Encryption Change
//! (Vol 4, Part E, 7.7.8) off, or with a failure, and Encryption Key
//! Refresh Complete (7.7.39) with a failure, each mean what this host sends
//! next on the link may go out in the clear. The link ends, and nothing
//! more is sent on it (`flush`), not even what already waits.

use super::session::Session;
use nonos_bt_wire::hci::EncryptionChange;

/// Encryption Key Refresh Complete's event code (7.7.39).
pub const KEY_REFRESH_COMPLETE: u8 = 0x30;
const ENCRYPTION_CHANGE: u8 = 0x08;

impl Session {
    pub(super) fn on_key_event(&mut self, code: u8, params: &[u8]) {
        let (status, handle, on) = match code {
            ENCRYPTION_CHANGE => match EncryptionChange::parse(params) {
                Ok(c) => (c.status, c.handle, c.enabled != 0),
                Err(_) => return,
            },
            _ => match params {
                [status, h0, h1] => (*status, u16::from_le_bytes([*h0, *h1]) & 0x0fff, true),
                _ => return,
            },
        };
        if handle == self.handle && (status != 0 || !on) {
            self.give_up("the controller says the link is no longer encrypted");
        }
    }
}
