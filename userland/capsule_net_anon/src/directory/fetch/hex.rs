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

//! Writing a fingerprint back out as hex for a URL.

/// Uppercase hex of `bytes` into `out`, returning what was written.
///
pub fn upper(out: &mut [u8], bytes: &[u8]) -> usize {
    const DIGITS: &[u8; 16] = b"0123456789ABCDEF";
    let mut at = 0usize;
    for byte in bytes {
        if at + 2 > out.len() {
            break;
        }
        out[at] = DIGITS[(byte >> 4) as usize];
        out[at + 1] = DIGITS[(byte & 0x0f) as usize];
        at += 2;
    }
    at
}
