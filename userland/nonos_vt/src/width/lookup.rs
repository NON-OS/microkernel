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

//! Looking a character up in the width tables.

use super::wide::WIDE;
use super::zero_high::ZERO_HIGH;
use super::zero_low::ZERO_LOW;

fn within(table: &[(u32, u32)], c: u32) -> bool {
    table
        .binary_search_by(|&(lo, hi)| {
            if hi < c {
                core::cmp::Ordering::Less
            } else if lo > c {
                core::cmp::Ordering::Greater
            } else {
                core::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

pub fn width(c: char) -> usize {
    let u = c as u32;
    if u < 0x300 {
        return 1;
    }
    if within(ZERO_LOW, u) || within(ZERO_HIGH, u) {
        0
    } else if within(WIDE, u) {
        2
    } else {
        1
    }
}

/// Zero width joiner: the character after it joins the one before.
pub const ZWJ: char = '\u{200D}';

/// Skin tone modifiers, which join the emoji before them.
pub fn is_emoji_modifier(c: char) -> bool {
    matches!(c as u32, 0x1F3FB..=0x1F3FF)
}
