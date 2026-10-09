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

use core::ops::Index;

use alloc::vec;
use alloc::vec::Vec;

use crate::browser::css::computed::Computed;

/* The computed style of every node, indexed by node id. Only elements
 * own a style: a text node (and any other non-element) reads its parent's,
 * which is the style its text inherits, so a page pays a slot index per
 * node and a full style per element. An id past the tree reads the root
 * default. */
pub struct Styles {
    slot: Vec<u32>,
    list: Vec<Computed>,
}

impl Styles {
    pub(in crate::browser::css) fn new(nodes: usize) -> Styles {
        Styles { slot: vec![0; nodes], list: vec![Computed::root()] }
    }

    /* Element `id` owns style `c`. */
    pub(in crate::browser::css) fn set(&mut self, id: usize, c: &Computed) {
        if let Some(s) = self.slot.get_mut(id) {
            *s = self.list.len() as u32;
            self.list.push(*c);
        }
    }

    /* Node `id` reads the style of `from`. */
    pub(in crate::browser::css) fn share(&mut self, id: usize, from: usize) {
        let s = self.slot.get(from).copied().unwrap_or(0);
        if let Some(d) = self.slot.get_mut(id) {
            *d = s;
        }
    }
}

impl Index<usize> for Styles {
    type Output = Computed;

    fn index(&self, id: usize) -> &Computed {
        &self.list[self.slot.get(id).copied().unwrap_or(0) as usize]
    }
}
