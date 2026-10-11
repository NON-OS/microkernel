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

//! What each of the client's ids names, and the version it was made at,
//! which bounds the events it may be sent. A global takes the version the
//! client binds (bind.rs); anything a request makes takes the version of
//! the object the request was sent to (object_version.rs).

use alloc::vec::Vec;

use super::{Object, MAX_OBJECTS};

pub struct Objects {
    pub(super) slots: Vec<(u32, Object, u32)>,
    /// The version of the object the request being served was sent to.
    pub(super) on: u32,
    /// Set once the client is told of a fatal error (object_life.rs).
    pub(super) failed: bool,
}

impl Objects {
    /// Object 1 is the display, always, before a client says anything.
    pub fn new() -> Objects {
        Objects { slots: alloc::vec![(1, Object::Display, 1)], on: 1, failed: false }
    }

    pub fn get(&self, id: u32) -> Option<Object> {
        self.slots.iter().find(|s| s.0 == id).map(|s| s.1)
    }

    /// Every id that names `what`, oldest first.
    pub fn all(&self, what: Object) -> impl Iterator<Item = u32> + '_ {
        self.slots.iter().filter(move |s| s.1 == what).map(|s| s.0)
    }

    /// Record `id` as `what`, made by the request being served; false,
    /// recording nothing, when a new id would pass MAX_OBJECTS or the
    /// client has failed, so the request that asked for it makes nothing.
    pub fn put(&mut self, id: u32, what: Object) -> bool {
        self.bound(id, what, self.on)
    }

    /// Record `id` as a global bound at `version`; false as for put.
    pub fn bound(&mut self, id: u32, what: Object, version: u32) -> bool {
        let full = self.slots.len() >= MAX_OBJECTS;
        if self.failed {
            return false;
        }
        match self.slots.iter_mut().find(|s| s.0 == id) {
            Some(slot) => *slot = (id, what, version),
            None if full => return false,
            None => self.slots.push((id, what, version)),
        }
        true
    }
}
