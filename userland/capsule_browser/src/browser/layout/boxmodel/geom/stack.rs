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

use super::stack_key::{digit, z_class, Key, DECOR, LAYER, LEVELS, ROOT};

/* Paint order (CSS 2.1 Appendix E) as one sortable key per fragment: a digit
 * per enclosing stacking context, outermost first. Within a context, at its
 * digit: the root box's background and borders, then children with a
 * negative z-index, the in-flow content, positioned boxes with z-index auto
 * or 0 in tree order, then positive z-index. A stable sort of the keys keeps
 * tree order among equals. Past LEVELS nested contexts a context paints
 * within its parent's. */
#[derive(Clone, Copy)]
pub(crate) struct Stack {
    /// The key the box's content paints with.
    pub key: Key,
    /* The enclosing context's key, its depth, and whether the box being
     * entered opened it (its own decoration then paints first). */
    base: Key,
    lvl: usize,
    fresh: bool,
}

impl Stack {
    pub(crate) const ROOT: Stack = Stack { key: ROOT, base: ROOT, lvl: 0, fresh: false };

    /// The stack inside the box at tree position `seq`: `z` its z-index (an
    /// integer opens a context), `layer` whether it is positioned.
    pub(crate) fn enter(self, z: Option<i32>, layer: bool, seq: usize) -> Stack {
        let lvl = self.lvl;
        match z {
            Some(z) if lvl < LEVELS => {
                let mut key = self.base;
                key[lvl] = digit(z_class(z), seq);
                Stack { key, base: key, lvl: lvl + 1, fresh: true }
            }
            None if layer && lvl < LEVELS => {
                let mut key = self.base;
                key[lvl] = digit(LAYER, seq);
                Stack { key, fresh: false, ..self }
            }
            _ => Stack { fresh: false, ..self },
        }
    }

    /// The key of the box's own background and borders.
    pub(crate) fn decor(self) -> Key {
        let mut key = self.key;
        if self.fresh && self.lvl < LEVELS {
            key[self.lvl] = DECOR;
        }
        key
    }
}
