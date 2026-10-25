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

//! What became of each sender's last loads, for a sender that stopped
//! waiting on a load's reply: the Terminal gives a load a short wait and
//! asks afterwards (OP_LOAD_OUTCOME), so a refusal still reaches it with
//! its reason. A fixed table, the oldest overwritten; a sender is told only
//! of its own loads.

/// Loads kept, across all senders.
pub const KEPT: usize = 8;
const NAME_MAX: usize = 64;

#[derive(Clone, Copy)]
struct Kept {
    sender: u32,
    name: [u8; NAME_MAX],
    len: usize,
    status: i32,
}

pub struct Outcomes {
    kept: [Option<Kept>; KEPT],
    next: usize,
}

impl Outcomes {
    pub const fn new() -> Self {
        Self { kept: [None; KEPT], next: 0 }
    }

    /// `sender`'s load of `name` ended in `status`; a newer load of the
    /// same name replaces the older.
    pub fn note(&mut self, sender: u32, name: &[u8], status: i32) {
        if name.len() > NAME_MAX {
            return;
        }
        let at = self.find(sender, name).unwrap_or_else(|| {
            let at = self.next;
            self.next = (at + 1) % KEPT;
            at
        });
        let mut kept = Kept { sender, name: [0; NAME_MAX], len: name.len(), status };
        kept.name[..name.len()].copy_from_slice(name);
        self.kept[at] = Some(kept);
    }

    /// How `sender`'s last load of `name` ended, if it is still kept.
    pub fn of(&self, sender: u32, name: &[u8]) -> Option<i32> {
        self.find(sender, name).and_then(|at| self.kept[at]).map(|k| k.status)
    }

    fn find(&self, sender: u32, name: &[u8]) -> Option<usize> {
        let same = |k: &Kept| k.sender == sender && &k.name[..k.len] == name;
        self.kept.iter().position(|k| k.as_ref().is_some_and(same))
    }
}
