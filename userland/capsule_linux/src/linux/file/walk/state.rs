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

/* Where a walk has got to, and the names of a path. */

use alloc::collections::VecDeque;
use alloc::vec::Vec;

/*
 * Where a walk has got to: the names still to walk, the names walked, and
 * how many links it has followed.
 */
pub(super) struct Walk {
    pub(super) todo: VecDeque<Vec<u8>>,
    pub(super) done: Vec<Vec<u8>>,
    pub(super) hops: usize,
}

impl Walk {
    /* Up one name; at the root there is none to go up from, so it stays. */
    pub(super) fn up(&mut self, path: &[u8]) {
        if self.done.pop().is_none() {
            super::super::clamp::note(path);
        }
    }

    /*
     * Into the link just walked: its target's names come next, from the
     * root for an absolute one.
     */
    pub(super) fn into_link(&mut self, to: &[u8]) {
        self.hops += 1;
        self.done.pop();
        if to.first() == Some(&b'/') {
            self.done.clear();
        }
        for (i, n) in names(to).enumerate() {
            self.todo.insert(i, n);
        }
    }
}

/* The names in `path`, with the empty ones and `.` left out. */
pub(super) fn names(path: &[u8]) -> impl Iterator<Item = Vec<u8>> + '_ {
    path.split(|b| *b == b'/').filter(|n| !n.is_empty() && *n != b".").map(<[u8]>::to_vec)
}

pub(super) fn joined(names: &[Vec<u8>]) -> Vec<u8> {
    if names.is_empty() {
        return alloc::vec![b'/'];
    }
    names.iter().flat_map(|n| [&b"/"[..], n].concat()).collect()
}
