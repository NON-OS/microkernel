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

//! The "+N" the tray ends with when some labels do not fit.

/// "+N", the count of labels left out.
pub(super) fn more_mark(n: usize, buf: &mut [u8; 12]) -> &[u8] {
    buf[0] = b'+';
    let mut digits = [0u8; 10];
    let mut i = digits.len();
    let mut v = n;
    loop {
        i -= 1;
        digits[i] = b'0' + (v % 10) as u8;
        v /= 10;
        if v == 0 {
            break;
        }
    }
    let d = digits.len() - i;
    buf[1..1 + d].copy_from_slice(&digits[i..]);
    &buf[..1 + d]
}
