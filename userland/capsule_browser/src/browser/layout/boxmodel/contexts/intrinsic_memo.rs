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

use core::cell::Cell;

/* A box's (min-content, max-content) border-box widths, worked out the
 * first time anything asks and answered from here after that. They depend
 * on the box's subtree alone, never on where it is laid, and the box tree
 * is built afresh for every layout pass, so a value lives exactly as long
 * as the pass that computed it. Without it nested flex and inline-block
 * boxes measured their subtrees again at every level above them. */
#[derive(Default)]
pub(in super::super) struct IntrinsicMemo(Cell<Option<(i32, i32)>>);

impl IntrinsicMemo {
    pub(in super::super) fn get_or(&self, compute: impl FnOnce() -> (i32, i32)) -> (i32, i32) {
        if let Some(v) = self.0.get() {
            return v;
        }
        let v = compute();
        self.0.set(Some(v));
        v
    }
}
