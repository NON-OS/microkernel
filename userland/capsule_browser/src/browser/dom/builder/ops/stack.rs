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

use super::state::{Builder, ON_STACK};

impl Builder {
    pub(in super::super) fn push_open(&mut self, id: usize) {
        let meta = self.meta_of(id);
        self.open.push(id);
        self.open_meta.push(meta);
        self.enter(id);
    }

    pub(in super::super) fn pop(&mut self) -> Option<usize> {
        let id = self.open.pop()?;
        self.open_meta.pop();
        self.leave(id);
        Some(id)
    }

    /// Take `id` out of the stack wherever it is.
    pub(in super::super) fn remove_open(&mut self, id: usize) {
        if let Some(i) = self.open.iter().rposition(|&x| x == id) {
            self.open.remove(i);
            self.open_meta.remove(i);
            self.leave(id);
        }
    }

    /// Put `new` where `old` is in the stack.
    pub(in super::super) fn replace_open(&mut self, old: usize, new: usize) {
        if let Some(i) = self.open.iter().rposition(|&x| x == old) {
            self.open[i] = new;
            self.open_meta[i] = self.meta_of(new);
            self.leave(old);
            self.enter(new);
        }
    }

    /// Put `id` into the stack right after `after`, towards the current node.
    pub(in super::super) fn insert_open_after(&mut self, after: usize, id: usize) {
        let at = self.open.iter().rposition(|&x| x == after).map_or(self.open.len(), |i| i + 1);
        self.open.insert(at, id);
        self.open_meta.insert(at, self.meta_of(id));
        self.enter(id);
    }

    pub(in super::super) fn on_stack(&self, id: usize) -> bool {
        self.flags[id] & ON_STACK != 0
    }

    fn enter(&mut self, id: usize) {
        self.flags[id] |= ON_STACK;
        self.count_name(id, true);
    }

    fn leave(&mut self, id: usize) {
        self.flags[id] &= !ON_STACK;
        self.count_name(id, false);
    }
}
