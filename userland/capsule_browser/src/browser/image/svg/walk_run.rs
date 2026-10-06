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

use alloc::vec;
use alloc::vec::Vec;

use super::state::Paint;
use super::walk::{container, skipped, Walk};
use super::xml::next_tag;

impl Walk<'_, '_> {
    /// Paint from `from` with inherited state `root`: every element to the
    /// end of the document, or when `one` is set just the element there
    /// and its subtree (a `use` target, which may be a symbol).
    pub(super) fn run(&mut self, from: usize, root: Paint, one: bool) {
        let mut stack: Vec<(Paint, bool)> = vec![(root, false)];
        let (mut pos, mut skip, mut open) = (from, 0u32, 0u32);
        while let Some((tag, next)) = next_tag(self.defs.doc, pos) {
            pos = next;
            if self.budget == 0 {
                return;
            }
            self.budget -= 1;
            if skip > 0 {
                skip = if tag.closing { skip - 1 } else { skip + u32::from(!tag.self_closing) };
                continue;
            }
            if tag.closing {
                if container(tag.name) && stack.len() > 1 && stack.pop().is_some_and(|s| s.1) {
                    self.masks.pop();
                }
                open = open.saturating_sub(1);
                if one && open == 0 {
                    return;
                }
                continue;
            }
            let first = one && open == 0;
            let draws = !skipped(tag.name) || (first && tag.name == "symbol");
            let cur = stack.last().map_or(root, |s| s.0);
            if !draws || !self.element(&tag, cur, &mut stack) {
                if first {
                    return;
                }
                skip = u32::from(!tag.self_closing);
                continue;
            }
            if one && tag.self_closing && open == 0 {
                return;
            }
            open += u32::from(!tag.self_closing);
        }
    }
}
