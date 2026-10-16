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
//! Which client enabled each slot: only it may name the slot (`dispatch`),
//! and the slots of one that exited are given back (`sweep`).
use super::types::SlotTable;
use super::valid::valid;

impl SlotTable {
    /// `slot_id`, just allocated, is `pid`'s.
    pub fn set_owner(&mut self, slot_id: u8, pid: u32) {
        if let Some(o) = self.owner.get_mut(slot_id as usize) {
            *o = pid;
        }
    }

    /// The client that enabled `slot_id`, while it is allocated; `None` for a
    /// free slot or one the kernel enabled.
    pub fn owner_of(&self, slot_id: u8, max_slots: u8) -> Option<u32> {
        let pid = *self.owner.get(slot_id as usize)?;
        (valid(slot_id, max_slots) && self.allocated[slot_id as usize] && pid != 0).then_some(pid)
    }

    /// Whether `slot_id` is allocated and `pid` enabled it; the kernel's
    /// own requests, pid 0, hold the slots they enabled.
    pub fn held_by(&self, slot_id: u8, max_slots: u8, pid: u32) -> bool {
        valid(slot_id, max_slots)
            && self.allocated[slot_id as usize]
            && self.owner[slot_id as usize] == pid
    }
}
