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

//! VCP's body: the value now and the most the control takes, u16 each,
//! then n u8 and the n values the capabilities list for the code, so the
//! one reading tells which inputs a display offers and which it shows.

use super::brightness_wire::EINVAL;

/// The status and the body's length, written into `out`.
pub fn reading((now, max): (u16, u16), listed: &[u8], out: &mut [u8]) -> (i32, usize) {
    let (Ok(n), Some(body)) = (u8::try_from(listed.len()), out.get_mut(..5 + listed.len())) else {
        return (EINVAL, 0);
    };
    body[..2].copy_from_slice(&now.to_le_bytes());
    body[2..4].copy_from_slice(&max.to_le_bytes());
    body[4] = n;
    body[5..].copy_from_slice(listed);
    (0, body.len())
}
