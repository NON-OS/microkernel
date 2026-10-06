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

//! REMOTE_NDIS_SET_MSG with a four-byte value (Remote NDIS 1.0, 2.2.6),
//! laid out as Linux generic_rndis_bind sets the packet filter: 28 bytes
//! of header, the value right after, its offset counted from RequestID.

use super::message::{put32, MSG_SET};

pub fn set_msg(oid: u32, value: u32) -> [u8; 32] {
    let mut m = [0u8; 32];
    for (at, v) in [(0, MSG_SET), (4, 32), (12, oid), (16, 4), (20, 20), (28, value)] {
        put32(&mut m, at, v);
    }
    m
}
