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
use super::bidi_l2::{mirror, order};
use super::bidi_levels::levels;

/* A line's items in visual order at their new line-relative x. Each space
 * is reordered as its own slot between the two items it separates (slot
 * 2k is item k, slot 2k-1 the space before it), at the lower of their two
 * levels: a space between runs of two directions takes the embedding
 * direction (UAX #9 N1, N2), so it stays between the runs. Slots reverse
 * from the highest level down to 1 (rule L2). A word at an odd level
 * reads right to left, drawn reversed with paired brackets mirrored. The
 * first item's space was dropped at the line start and stays dropped. */
pub(in super::super) fn visual(items: Vec<(i32, InlineItem)>, rtl: bool) -> Vec<(i32, InlineItem)> {
    let lv = levels(&items, rtl as u8);
    let seq: Vec<u8> =
        (0..(2 * lv.len()).saturating_sub(1)).map(|j| lv[j / 2].min(lv[(j + 1) / 2])).collect();
    let sp: Vec<i32> = items.iter().map(|(_, it)| it.space_w()).collect();
    let mut slots: Vec<Option<InlineItem>> = items.into_iter().map(|(_, it)| Some(it)).collect();
    let (mut x, mut out) = (0i32, Vec::with_capacity(slots.len()));
    for j in order(&seq) {
        let i = (j + 1) / 2;
        if j % 2 == 1 {
            x += sp[i];
            continue;
        }
        let Some(mut it) = slots.get_mut(i).and_then(Option::take) else { continue };
        if let (1, InlineItem::Word { text, .. }) = (lv[i] % 2, &mut it) {
            *text = text.chars().rev().map(mirror).collect();
        }
        let w = it.advance_w();
        out.push((x, it));
        x += w;
    }
    out
}
