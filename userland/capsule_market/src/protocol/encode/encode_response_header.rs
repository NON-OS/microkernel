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

use crate::protocol::header::{Request, MAGIC, VERSION};

/// None when `out` is shorter than a response header.
pub(crate) fn encode_response_header(
    out: &mut [u8],
    req: &Request,
    payload_len: u32,
) -> Option<()> {
    out.get_mut(0..4)?.copy_from_slice(&MAGIC.to_le_bytes());
    out.get_mut(4..6)?.copy_from_slice(&VERSION.to_le_bytes());
    out.get_mut(6..8)?.copy_from_slice(&req.op.to_le_bytes());
    out.get_mut(8..10)?.copy_from_slice(&req.flags.to_le_bytes());
    out.get_mut(10..12)?.copy_from_slice(&0u16.to_le_bytes());
    out.get_mut(12..16)?.copy_from_slice(&req.request_id.to_le_bytes());
    out.get_mut(16..20)?.copy_from_slice(&payload_len.to_le_bytes());
    Some(())
}
