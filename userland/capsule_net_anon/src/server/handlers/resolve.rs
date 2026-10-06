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


//! Turning a short .anyone name into the address it stands for.

use crate::manager::{resolve_name, Manager, SendError};
use crate::onion::address::encode;
use crate::onion::names::NAME_NOTICE;
use crate::protocol::{E_BAD_LEN, E_BAD_ONION, E_NAMES_PENDING, E_NAME_CHANGED, E_NAME_UNKNOWN, E_OK, E_TABLE_FULL, HDR_LEN};

/// Request body: the name. Reply body: the 63-byte address, a newline, and
/// NAME_NOTICE, the sentence a caller shows wherever it uses the name, so
/// the user is told a short name is weaker than the address.
pub fn resolve(state: &mut Manager, body: &[u8], now: u64, tx: &mut [u8]) -> (u16, u32) {
    if body.is_empty() || body.len() > 255 {
        return (E_BAD_LEN, 0);
    }
    match resolve_name(state, body, now) {
        Ok(identity) => {
            let address = encode(&identity);
            let notice = NAME_NOTICE.as_bytes();
            let len = address.len() + 1 + notice.len();
            let Some(out) = tx.get_mut(HDR_LEN..HDR_LEN + len) else {
                return (E_BAD_LEN, 0);
            };
            out[..address.len()].copy_from_slice(&address);
            out[address.len()] = b'\n';
            out[address.len() + 1..].copy_from_slice(notice);
            (E_OK, len as u32)
        }
        Err(SendError::NamesPending) => (E_NAMES_PENDING, 0),
        Err(SendError::NameUnknown) => (E_NAME_UNKNOWN, 0),
        Err(SendError::NameChanged) => (E_NAME_CHANGED, 0),
        Err(SendError::TableFull) => (E_TABLE_FULL, 0),
        Err(_) => (E_BAD_ONION, 0),
    }
}
