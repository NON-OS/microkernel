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

//! Finding the handshake reply inside an EXTENDED2 body.

/// The handshake reply an EXTENDED2 body carries.
///
pub fn extended2_reply(body: &[u8]) -> Option<&[u8]> {
    if body.len() < 2 {
        return None;
    }
    let length = u16::from_be_bytes([body[0], body[1]]) as usize;
    body.get(2..2 + length)
}
