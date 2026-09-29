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

use super::super::inline_items::InlineItem;
use super::bidi_levels::levels;

/* A line's items in visual order at their new line-relative x. Runs are
 * reversed from the highest level down to 1 (UAX #9 rule L2). A word at
 * an odd level reads right to left, so its characters are drawn reversed
 * with paired brackets mirrored, and the space before it in logical order
 * falls on its right. The first item's space was dropped at the line
 * start and stays dropped. */
pub(in super::super) fn visual(items: Vec<(i32, InlineItem)>, rtl: bool) -> Vec<(i32, InlineItem)> {
    let lv = levels(&items, rtl as u8);
    let mut slots: Vec<Option<InlineItem>> = items.into_iter().map(|(_, it)| Some(it)).collect();
    let (mut x, mut out) = (0i32, Vec::with_capacity(slots.len()));
    for i in order(&lv) {
        let Some(mut it) = slots.get_mut(i).and_then(Option::take) else { continue };
        let space = if i == 0 { 0 } else { it.space_w() };
        let odd = lv[i] % 2 == 1;
        if let (true, InlineItem::Word { text, .. }) = (odd, &mut it) {
            *text = text.chars().rev().map(mirror).collect();
        }
        out.push((if odd { x } else { x + space }, it));
        x += space + out.last().map_or(0, |(_, it)| it.advance_w());
    }
    out
}

/* Logical indices in visual order for the given levels. */
fn order(lv: &[u8]) -> Vec<usize> {
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
fn mirror(ch: char) -> char {
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
