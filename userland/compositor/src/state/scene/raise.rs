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

use super::layer::{Layer, MAX_LAYERS};
use super::table::SceneTable;

impl SceneTable {
    // Put each of `owner_pid`'s layers on top of its own z band, and hand
    // `past` every layer a raised one rose over, with it: only where the two
    // meet can a pixel change, since everywhere else the layer on top at a
    // point is the one that was. A layer already on top of its band rose
    // over nothing. Nothing is handed back when the project has no layer
    // yet; its first submit puts it on top anyway.
    pub fn raise(&mut self, owner_pid: u32, past: impl FnMut(&Layer, &Layer)) {
        self.raise_where(|l| l.owner_pid == owner_pid, past);
    }

    // Put the layers of one window on top of their bands: every layer of
    // `owner_pid` tagged `window_id`. An owner that never tags its layers (a
    // client that submits the old way) has its untagged layers raised, which
    // for such a client is every layer it holds, as raise does.
    pub fn raise_window(
        &mut self,
        owner_pid: u32,
        window_id: u32,
        past: impl FnMut(&Layer, &Layer),
    ) {
        let tagged = window_id != 0
            && self.layers().any(|l| l.owner_pid == owner_pid && l.window_id == window_id);
        let id = if tagged { window_id } else { 0 };
        self.raise_where(|l| l.owner_pid == owner_pid && l.window_id == id, past);
    }

    // Raise every layer `member` picks to the top of its band, keeping the
    // order they had among themselves: a window's dialogs stay over it when it
    // comes up. A band where no other layer stands over any of them is left
    // as it is. Each raised layer is handed to `past` with every other layer
    // of its band that stood over it, before any stamp changes.
    fn raise_where(
        &mut self,
        member: impl Fn(&Layer) -> bool,
        mut past: impl FnMut(&Layer, &Layer),
    ) {
        let mut order = [(0u32, 0usize); MAX_LAYERS];
        let mut n = 0;
        for (i, l) in self.entries.iter().enumerate() {
            if l.in_use && member(l) {
                order[n] = (l.stack, i);
                n += 1;
            }
        }
        order[..n].sort_unstable();
        let covered = |entries: &[Layer], z: u32| {
            let lowest = order[..n].iter().map(|&(_, i)| entries[i]).filter(|l| l.z == z);
            let Some(floor) = lowest.map(|l| l.stack).min() else { return false };
            entries.iter().any(|l| l.in_use && !member(l) && l.z == z && l.stack > floor)
        };
        let mut lift = [false; MAX_LAYERS];
        for &(_, i) in order[..n].iter() {
            lift[i] = covered(&self.entries, self.entries[i].z);
        }
        for &(_, i) in order[..n].iter().filter(|&&(_, i)| lift[i]) {
            let m = &self.entries[i];
            let over = |l: &&Layer| l.in_use && !member(l) && l.z == m.z && l.stack > m.stack;
            self.entries.iter().filter(over).for_each(|l| past(m, l));
        }
        for &(_, i) in order[..n].iter() {
            if !lift[i] {
                continue;
            }
            let stack = self.take_stack();
            self.entries[i].stack = stack;
        }
    }

    // The next raise stamp. The counter only runs out after four billion raises;
    // before it would wrap, the live stamps are renumbered 1..=n in their
    // current order, so the order survives and the counter starts low again.
    pub(super) fn take_stack(&mut self) -> u32 {
        if self.next_stack == u32::MAX {
            self.renumber();
        }
        let stack = self.next_stack;
        self.next_stack += 1;
        stack
    }

    fn renumber(&mut self) {
        let mut order = [(0u32, 0usize); MAX_LAYERS];
        let mut n = 0;
        for (i, l) in self.entries.iter().enumerate() {
            if l.in_use {
                order[n] = (l.stack, i);
                n += 1;
            }
        }
        order[..n].sort_unstable();
        for (rank, &(_, i)) in order[..n].iter().enumerate() {
            self.entries[i].stack = rank as u32 + 1;
        }
        self.next_stack = n as u32 + 1;
    }

    #[cfg(test)]
    pub fn set_next_stack(&mut self, next: u32) {
        self.next_stack = next;
    }
}
