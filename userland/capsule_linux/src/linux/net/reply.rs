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

//! A net.sockets reply as the receive buffer holds it: the status in the
//! header and the payload after it, sliced no further than the buffer,
//! whatever length the call reports.

/// Status is a u16 at offset 8 of the reply header.
const STATUS_AT: usize = 8;

/// `n` is what the call returned; anything shorter than the header, or
/// negative, is no reply.
pub fn reply_parts(rx: &[u8], n: i64, hdr_len: usize) -> Option<(u16, &[u8])> {
    let n = usize::try_from(n).ok()?;
    if n < hdr_len || hdr_len < STATUS_AT + 2 || rx.len() < hdr_len {
        return None;
    }
    let status = u16::from_le_bytes([rx[STATUS_AT], rx[STATUS_AT + 1]]);
    Some((status, &rx[hdr_len..n.min(rx.len())]))
}
