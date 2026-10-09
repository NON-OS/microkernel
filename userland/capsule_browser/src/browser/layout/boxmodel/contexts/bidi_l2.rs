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

use alloc::vec::Vec;

/* Logical indices in visual order for the given levels (UAX #9 rule L2):
 * from the highest level down to 1, each run at that level or above is
 * reversed. */
pub(super) fn order(lv: &[u8]) -> Vec<usize> {
    let mut ord: Vec<usize> = (0..lv.len()).collect();
    for level in (1..=lv.iter().copied().max().unwrap_or(0)).rev() {
        let mut i = 0;
        while i < ord.len() {
            let start = i;
            while i < ord.len() && lv[ord[i]] >= level {
                i += 1;
            }
            ord[start..i].reverse();
            i += 1;
        }
    }
    ord
}

/* The mirrored glyph of a paired bracket, as right-to-left text shows it. */
pub(super) fn mirror(ch: char) -> char {
    match ch {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        c => c,
    }
}
