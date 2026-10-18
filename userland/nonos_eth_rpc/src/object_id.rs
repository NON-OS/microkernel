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

/// Whether a response object carries `"id":<id>`, matching the whole number so
/// id 2 is not mistaken for the 2 in id 20.
pub(crate) fn object_has_id(obj: &[u8], id: u64) -> bool {
    let pat = b"\"id\":";
    let mut base = 0usize;
    while let Some(pos) = obj[base..].windows(pat.len()).position(|w| w == pat) {
        let mut j = base + pos + pat.len();
        while j < obj.len() && obj[j] == b' ' {
            j += 1;
        }
        let mut val = 0u64;
        let mut any = false;
        while j < obj.len() && obj[j].is_ascii_digit() {
            val = val.wrapping_mul(10).wrapping_add((obj[j] - b'0') as u64);
            any = true;
            j += 1;
        }
        if any && val == id {
            return true;
        }
        base += pos + pat.len();
    }
    false
}
