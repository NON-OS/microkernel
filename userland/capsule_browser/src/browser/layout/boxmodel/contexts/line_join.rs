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

/* Put a word cut for its break opportunities back together where one line
 * holds the pieces: "well-" and "known" side by side paint as the one
 * word "well-known" they are. A piece that starts a line stays apart. */
pub(in super::super) fn join_pieces(items: Vec<(i32, InlineItem)>) -> Vec<(i32, InlineItem)> {
    let mut out: Vec<(i32, InlineItem)> = Vec::with_capacity(items.len());
    for (x, item) in items {
        let joins = matches!(&item, InlineItem::Word { lead, .. } if lead.join);
        if let (true, Some((px, InlineItem::Word { text, adv, .. }))) = (joins, out.last_mut()) {
            if let InlineItem::Word { text: more, adv: w, .. } = &item {
                text.push_str(more);
                *adv = (x + w - *px).max(*adv);
                continue;
            }
        }
        out.push((x, item));
    }
    out
}
