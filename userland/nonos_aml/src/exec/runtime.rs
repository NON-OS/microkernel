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

//! What evaluations leave behind for the next one.

use super::{BufferField, Personality, Region};
use crate::namespace::Id;
use crate::object::Object;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;

/// A mutex's holder: which evaluation, and how many times it acquired it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Holder {
    pub eval: u64,
    pub depth: u32,
}

#[derive(Debug, Default)]
pub struct Runtime {
    /// Name values by entry id; None until first read or stored.
    pub values: Vec<Option<Object>>,
    /// Parsed method bodies, by (table, Method node).
    pub bodies: BTreeMap<(u16, u32), u32>,
    pub mutexes: BTreeMap<Id, Holder>,
    pub events: BTreeMap<Id, u32>,
    /// Notify's (object, value) pairs, oldest first, for the capsule.
    pub notifications: VecDeque<(Id, u8)>,
    pub dropped_notifications: u32,
    pub evaluations: u64,
    pub personality: Personality,
    pub regions: BTreeMap<Id, Region>,
    pub buffer_fields: BTreeMap<Id, BufferField>,
    /// Spaces whose handler is connected, one bit each (Space::bit).
    pub connected: u32,
    /// How deep this capsule holds the Global Lock.
    pub global_lock: u32,
}

impl Runtime {
    /// Notifications beyond this are dropped and counted: a table cannot
    /// grow the queue while the capsule is not draining it.
    pub const MAX_NOTIFICATIONS: usize = 64;

    /// Everything kept about an object, dropped when the object is removed.
    pub fn forget(&mut self, id: Id) {
        *self.value(id) = None;
        self.regions.remove(&id);
        self.buffer_fields.remove(&id);
    }

    pub fn value(&mut self, id: Id) -> &mut Option<Object> {
        if self.values.len() <= id as usize {
            self.values.resize(id as usize + 1, None);
        }
        &mut self.values[id as usize]
    }
}
