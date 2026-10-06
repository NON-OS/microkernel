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

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::browser::css::pseudo_style::PseudoText;

/* The generated boxes and pseudo-element styles of the elements that have
 * any, sorted by element id: indexing an element gives its list, empty
 * for the many elements with none, which take no space. */
#[derive(Default)]
pub struct Pseudos {
    ids: Vec<u32>,
    items: Vec<PseudoText>,
}

impl Pseudos {
    /* From (element id, box) pairs in any element order; the boxes of one
     * element keep the order they were pushed in. */
    pub(in crate::browser::css) fn from(mut pairs: Vec<(u32, Box<PseudoText>)>) -> Pseudos {
        pairs.sort_by_key(|p| p.0);
        let mut ids = Vec::with_capacity(pairs.len());
        let mut items = Vec::with_capacity(pairs.len());
        for (id, p) in pairs {
            ids.push(id);
            items.push(*p);
        }
        Pseudos { ids, items }
    }
}

impl Index<usize> for Pseudos {
    type Output = [PseudoText];

    fn index(&self, id: usize) -> &[PseudoText] {
        let id = u32::try_from(id).unwrap_or(u32::MAX);
        let a = self.ids.partition_point(|&k| k < id);
        let z = self.ids.partition_point(|&k| k <= id);
        &self.items[a..z]
    }
}
