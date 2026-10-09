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

use super::super::inline_items::InlineItem;

pub(in super::super) use super::bidi_order::visual;

/* A line item's bidi class, from its first strong character (UAX #9
 * reduced to what a line of words needs): left-to-right, right-to-left
 * (Hebrew, Arabic and their neighbours), a number, or neutral. */
#[derive(Clone, Copy, PartialEq, Eq)]
pub(in super::super) enum Class {
    L,
    R,
    Num,
    N,
}

fn is_rtl(ch: char) -> bool {
    matches!(ch as u32, 0x0590..=0x08FF | 0xFB1D..=0xFDFF | 0xFE70..=0xFEFF | 0x1EE00..=0x1EEFF)
}

pub(in super::super) fn class(item: &InlineItem) -> Class {
    let InlineItem::Word { text, .. } = item else { return Class::N };
    let mut num = false;
    for ch in text.chars() {
        if is_rtl(ch) {
            return Class::R;
        }
        if ch.is_alphabetic() {
            return Class::L;
        }
        num |= ch.is_numeric();
    }
    if num {
        Class::Num
    } else {
        Class::N
    }
}

/* Whether a line needs reordering: its paragraph runs right to left, or a
 * word in it does. A plain left-to-right line skips all of this. */
pub(in super::super) fn needs_bidi(rtl: bool, items: &[(i32, InlineItem)]) -> bool {
    rtl || items
        .iter()
        .any(|(_, it)| matches!(it, InlineItem::Word { text, .. } if text.chars().any(is_rtl)))
}
