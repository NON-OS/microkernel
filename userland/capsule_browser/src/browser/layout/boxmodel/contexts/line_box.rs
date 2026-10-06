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

use super::super::display_list::DisplayList;
use super::super::flush_line::Place;
use super::super::inline_items::InlineItem;

/* The line being filled: its items at their line-relative x, the width
 * used, the tallest item, and the last place it may break. */
pub(in super::super) struct LineBox {
    items: Vec<(i32, InlineItem)>,
    pub(in super::super) w: i32,
    h: i32,
    base_h: i32,
    /* Index of the last item a break may come before (never the first). */
    last_brk: Option<usize>,
}

impl LineBox {
    pub(in super::super) fn new(base_h: i32) -> Self {
        LineBox { items: Vec::new(), w: 0, h: 0, base_h, last_brk: None }
    }

    pub(in super::super) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /* Append an item; the space before it is dropped at the line start. */
    pub(in super::super) fn push(&mut self, item: InlineItem) {
        let lead = item.lead();
        let x = if self.is_empty() { 0 } else { self.w.saturating_add(lead.space) };
        if lead.brk && !self.is_empty() {
            self.last_brk = Some(self.items.len());
        }
        self.h = self.h.max(item.item_h());
        self.w = x.saturating_add(item.advance_w());
        self.items.push((x, item));
    }

    /* Cut the line at its last break opportunity and hand back what
     * follows it, for the next line. None when it has no opportunity. */
    pub(in super::super) fn take_tail(&mut self) -> Option<Vec<InlineItem>> {
        let at = self.last_brk.take()?;
        let tail = self.items.split_off(at).into_iter().map(|(_, it)| it).collect();
        let kept = core::mem::take(&mut self.items);
        (self.w, self.h) = (0, 0);
        kept.into_iter().for_each(|(_, it)| self.push(it));
        Some(tail)
    }

    /* Emit the line at `top` and return its height; the next line starts
     * empty. A line with nothing on it (two breaks in a row) still takes
     * the container's line height. */
    pub(in super::super) fn flush(&mut self, out: &mut DisplayList, p: &Place, top: i32) -> i32 {
        let h = self.h.max(self.base_h);
        p.emit(out, core::mem::take(&mut self.items), [top, h, self.w]);
        (self.w, self.h, self.last_brk) = (0, 0, None);
        h
    }
}
