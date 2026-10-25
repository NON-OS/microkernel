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

//! Every reference a PPTT makes, checked before anyone follows one: a
//! parent names a processor node, a private resource names a structure, a
//! next level names a cache, and no chain of parents or of cache levels
//! loops. The walk records structures in offset order, so a reference is
//! found by binary search, and each chain is walked once: a hostile table
//! costs time in its length, not in its length cubed.

use super::super::sdt::TableError;
use super::looping::looping;
use super::walk::Pptt;

pub fn references(p: &Pptt, others: &[u32]) -> Result<(), TableError> {
    let node = |at: u32| p.nodes.binary_search_by_key(&at, |n| n.offset).ok();
    let cache = |at: u32| p.caches.binary_search_by_key(&at, |c| c.offset).ok();
    for n in &p.nodes {
        let bad = TableError::Reference(n.offset as usize);
        if n.parent.is_some_and(|at| node(at).is_none()) {
            return Err(bad);
        }
        let known =
            |r: &u32| cache(*r).is_some() || node(*r).is_some() || others.binary_search(r).is_ok();
        if !n.resources.iter().all(known) {
            return Err(bad);
        }
    }
    for c in &p.caches {
        if c.next_level.is_some_and(|at| cache(at).is_none()) {
            return Err(TableError::Reference(c.offset as usize));
        }
    }
    let up = |i: usize| p.nodes[i].parent.and_then(node);
    if let Some(i) = looping(p.nodes.len(), up) {
        return Err(TableError::Reference(p.nodes[i].offset as usize));
    }
    let down = |i: usize| p.caches[i].next_level.and_then(cache);
    match looping(p.caches.len(), down) {
        Some(i) => Err(TableError::Reference(p.caches[i].offset as usize)),
        None => Ok(()),
    }
}
