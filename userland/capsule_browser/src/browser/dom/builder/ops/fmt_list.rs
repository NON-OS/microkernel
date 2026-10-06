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

use super::mode::Entry;
use super::state::{Builder, IN_FMT};

impl Builder {
    pub(in super::super) fn in_fmt(&self, id: usize) -> bool {
        self.flags[id] & IN_FMT != 0
    }

    pub(in super::super) fn fmt_index(&self, id: usize) -> Option<usize> {
        if !self.in_fmt(id) {
            return None;
        }
        self.fmt.iter().rposition(|e| *e == Entry::Elem(id))
    }

    /// Take `id` out of the list, if it is there.
    pub(in super::super) fn remove_fmt(&mut self, id: usize) {
        if let Some(i) = self.fmt_index(id) {
            self.fmt.remove(i);
        }
        self.flags[id] &= !IN_FMT;
    }

    /// Put `new` in the list where `old` is.
    pub(in super::super) fn replace_fmt(&mut self, at: usize, old: usize, new: usize) {
        self.fmt[at] = Entry::Elem(new);
        self.flags[old] &= !IN_FMT;
        self.fmt_mark(new);
    }

    /// Clear the list of active formatting elements up to the last marker.
    pub(in super::super) fn clear_to_marker(&mut self) {
        while let Some(e) = self.fmt.pop() {
            match e {
                Entry::Marker => return,
                Entry::Elem(id) => self.flags[id] &= !IN_FMT,
            }
        }
    }
}
