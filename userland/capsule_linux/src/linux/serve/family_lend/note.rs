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

//! The note a lent guest gets: which slots and pipe ends a descriptor
//! anywhere in the family still names.

use alloc::vec::Vec;

use super::super::family::Family;
use crate::linux::guest::Kind;

impl Family {
    /// For each of `len` slots, whether a descriptor of `kind` anywhere in
    /// the family names it.
    pub(super) fn named(&self, kind: Kind, len: usize) -> Vec<bool> {
        let mut used = alloc::vec![false; len];
        let open = self.guests.iter().flat_map(|g| g.fds.iter());
        for f in open.filter(|f| f.kind == kind) {
            if let Some(u) = used.get_mut(f.handle as usize) {
                *u = true;
            }
        }
        used
    }

    /// For each pipe: whether a read end is open, whether a write end is.
    pub(super) fn pipe_ends(&self) -> Vec<(bool, bool)> {
        let mut ends = alloc::vec![(false, false); self.pipes.len()];
        let open = self.guests.iter().flat_map(|g| g.fds.iter());
        for f in open.filter(|f| f.kind == Kind::Pipe) {
            if let Some(end) = ends.get_mut(f.handle as usize) {
                match f.writable {
                    true => end.1 = true,
                    false => end.0 = true,
                }
            }
        }
        ends
    }
}
