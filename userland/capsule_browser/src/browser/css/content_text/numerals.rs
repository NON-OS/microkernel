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

use alloc::string::String;
use alloc::vec::Vec;

/* `v` (at least 1) in letters from `base`: a..z, then aa, ab and on. */
pub(super) fn alpha(mut v: i32, base: u8) -> String {
    let mut digits = Vec::new();
    while v > 0 {
        v -= 1;
        digits.push(base + (v % 26) as u8);
        v /= 26;
    }
    digits.iter().rev().map(|&b| b as char).collect()
}

/* `v` (1 to 3999) in upper-case Roman numerals. */
pub(super) fn roman(mut v: i32) -> String {
    const N: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    const S: [&str; 13] = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    let mut out = String::new();
    for (n, s) in N.into_iter().zip(S) {
        while v >= n {
            out.push_str(s);
            v -= n;
        }
    }
    out
}
