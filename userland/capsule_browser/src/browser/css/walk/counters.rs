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

mod element;
mod ops;

use alloc::vec::Vec;

use crate::browser::css::rule_index::key::hash_name;

/* Counter instances alive at one point of the walk. */
pub(super) const MAX_COUNTERS: usize = 1024;

/* CSS counters in document order. A counter-reset on an element creates
 * an instance that its later siblings and all their descendants see, so
 * each instance belongs to the parent of the element that made it and
 * ends when that parent's children are done. The quote depth that
 * open-quote and close-quote move lives here too. */
#[derive(Default)]
pub(in crate::browser::css) struct Counters {
    /* (name hash, value, parent element whose children see it). */
    stack: Vec<(u64, i32, usize)>,
    pub quotes: u32,
}

impl Counters {
    /* The children of `id` are done: the instances they made end. */
    pub fn leave(&mut self, id: usize) {
        while self.stack.last().is_some_and(|c| c.2 == id) {
            self.stack.pop();
        }
    }

    /* Every instance of `name`, outermost first (counters()), the
     * innermost last (counter()). */
    pub fn values(&self, name: &str) -> impl Iterator<Item = i32> + '_ {
        let h = hash_name(name, true);
        self.stack.iter().filter(move |c| c.0 == h).map(|c| c.1)
    }
}
