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

//! "N listed", built without a formatter because this is `no_std`.

/// Right-aligned in a fixed field so the head band does not reflow as the
/// count changes.
pub fn listed(n: usize) -> [u8; 16] {
    let mut out = *b"       0 listed ";
    let mut at = 8;
    let mut left = n;
    loop {
        at -= 1;
        out[at] = b'0' + (left % 10) as u8;
        left /= 10;
        if left == 0 || at == 0 {
            break;
        }
    }
    out
}
