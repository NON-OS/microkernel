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

//! The (status, association id) of an association response, or `None` if
//! the frame is not one.

use super::*;

/// The (status, association id) of an association response, or `None` if the
/// frame is not one. Zero status is success; the AID is the low 14 bits. The
/// body after the MAC header is capability (2), status (2), AID (2).
pub fn parse_assoc_response(frame: &[u8]) -> Option<(u16, u16)> {
    if frame.len() < MAC_HEADER_LEN + 6 {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    if fc_type(fc) != TYPE_MGMT || fc_subtype(fc) != super::super::header::SUBTYPE_ASSOC_RESP {
        return None;
    }
    let status = u16::from_le_bytes([frame[MAC_HEADER_LEN + 2], frame[MAC_HEADER_LEN + 3]]);
    let aid = u16::from_le_bytes([frame[MAC_HEADER_LEN + 4], frame[MAC_HEADER_LEN + 5]]) & 0x3FFF;
    Some((status, aid))
}
