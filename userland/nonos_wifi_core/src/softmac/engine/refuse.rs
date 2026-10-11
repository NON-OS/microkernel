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

//! The answer to a frame neither request family takes. Its caller is blocked
//! until a reply comes, so it gets one: the link family's shape, a header and
//! an invalid-request status, naming the op and request id the frame's first
//! octets carry, or zeros when it is shorter.

use crate::netif::wire;

pub const STATUS_REFUSED: i32 = -22;

pub fn refuse(req: &[u8], out: &mut [u8]) -> usize {
    let (op, request_id) = match req.first_chunk::<{ wire::HDR_LEN }>() {
        Some(h) => {
            (u16::from_le_bytes([h[6], h[7]]), u32::from_le_bytes([h[12], h[13], h[14], h[15]]))
        }
        None => (0, 0),
    };
    let Some(status) = out.get_mut(wire::HDR_LEN..wire::HDR_LEN + 4) else {
        return 0;
    };
    status.copy_from_slice(&STATUS_REFUSED.to_le_bytes());
    wire::write_response_header(out, op, request_id, 4).map_or(0, |n| n + 4)
}
