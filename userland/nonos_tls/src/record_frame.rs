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

//! Where one record starts and ends, before anything is decrypted.

/// Content type for records sealed under traffic keys, RFC 8446 section 5.
pub const APPLICATION_DATA: u8 = 23;

/// The type of the whole record that starts at `at`, and the offset just past
/// it. `None` while its header or any of its body is still arriving.
pub fn record_at(bytes: &[u8], at: usize) -> Option<(u8, usize)> {
    let head = bytes.get(at..at.checked_add(5)?)?;
    let len = u16::from_be_bytes([head[3], head[4]]) as usize;
    let end = at + 5 + len;
    if end > bytes.len() {
        return None;
    }
    Some((head[0], end))
}
