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

/// How many bytes the last character of `bytes` takes, what one Backspace
/// removes from a field that holds UTF-8. Zero for an empty field.
pub fn last_char_len(bytes: &[u8]) -> usize {
    let mut i = bytes.len();
    while i > 0 {
        i -= 1;
        if bytes[i] & 0xC0 != 0x80 {
            break;
        }
    }
    bytes.len() - i
}

/// Append `ch` to the `len` bytes held in `buf`, as long as it fits within
/// `cap` bytes whole. Returns the new length, or None when it does not fit.
pub fn push_char(buf: &mut [u8], len: usize, cap: usize, ch: char) -> Option<usize> {
    let mut enc = [0u8; 4];
    let bytes = ch.encode_utf8(&mut enc).as_bytes();
    let end = len.checked_add(bytes.len())?;
    if end > cap.min(buf.len()) {
        return None;
    }
    buf[len..end].copy_from_slice(bytes);
    Some(end)
}
