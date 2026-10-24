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
//! A slot given back by one of the drivers sharing it.

use super::claims::Claims;

/// What a driver giving a slot back leaves.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Release {
    /// No interface of it was claimed: disabled, as ever.
    Unclaimed,
    /// The caller's were the last: disable it.
    Last,
    /// Another live driver still holds one: the slot stays.
    Kept,
    /// The caller holds none and another does: refused.
    NotHolder,
}

impl Claims {
    /// `pid` gives `slot` back; claims of drivers `alive` says are gone go too.
    pub fn release(&mut self, slot: u8, pid: u32, alive: impl Fn(u32) -> bool) -> Release {
        let had = self.list.iter().filter(|c| c.slot == slot).count();
        let mine = self.list.iter().any(|c| c.slot == slot && c.pid == pid);
        self.list.retain(|c| c.slot != slot || (c.pid != pid && alive(c.pid)));
        let left = self.list.iter().any(|c| c.slot == slot);
        match (had, mine, left) {
            (0, _, _) => Release::Unclaimed,
            (_, _, false) => Release::Last,
            (_, true, true) => Release::Kept,
            (_, false, true) => Release::NotHolder,
        }
    }
}
