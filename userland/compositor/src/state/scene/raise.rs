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
    // Put each of `owner_pid`'s layers on top of its own z band, and hand every
    // layer that raise reordered to `raised`, so the caller repaints its
    // rectangle: when a layer moves up past its peers only pixels inside it can
    // change, and a layer already on top of its band changes nothing. Nothing
    // is handed back when the owner has no layer yet; its first submit puts it
    // on top anyway. An owner holds one layer per band, so its layers never
    // compete with each other here.
    pub fn raise(&mut self, owner_pid: u32, mut raised: impl FnMut(&Layer)) {
        for at in 0..MAX_LAYERS {
            let me = self.entries[at];
            if !me.in_use || me.owner_pid != owner_pid {
                continue;
            }
            let covered = self
                .entries
                .iter()
                .any(|l| l.in_use && l.owner_pid != owner_pid && l.z == me.z && l.stack > me.stack);
            if !covered {
                continue;
            }
            let stack = self.take_stack();
            self.entries[at].stack = stack;
            raised(&self.entries[at]);
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
