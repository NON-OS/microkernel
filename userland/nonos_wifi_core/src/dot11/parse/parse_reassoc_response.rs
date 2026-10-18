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

//! The (status, association id) of a reassociation response (IEEE Std
//! 802.11-2020, 9.3.3.8). Its body opens as an association response's
//! does: capability, status, AID.

use super::*;
use crate::dot11::header::SUBTYPE_REASSOC_RESP;

pub fn parse_reassoc_response(frame: &[u8]) -> Option<(u16, u16)> {
    let fc = u16::from_le_bytes([*frame.first()?, *frame.get(1)?]);
    if fc_type(fc) != TYPE_MGMT || fc_subtype(fc) != SUBTYPE_REASSOC_RESP {
        return None;
    }
    let body = frame.get(MAC_HEADER_LEN..MAC_HEADER_LEN + 6)?;
    let status = u16::from_le_bytes([body[2], body[3]]);
    Some((status, u16::from_le_bytes([body[4], body[5]]) & 0x3FFF))
}
