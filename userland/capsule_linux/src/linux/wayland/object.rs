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


//! The client's object table.

use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Display,
    Registry,
    Callback,
    Compositor,
    Shm,
    ShmPool,
    Buffer,
    Surface,
    XdgBase,
    XdgSurface,
    XdgToplevel,
    Seat,
    Pointer,
    Keyboard,
    Output,
}

/// The most objects one client holds at once. A real client holds a few
/// hundred; without a ceiling, a client creating objects it never destroys
/// would grow this capsule's memory, and slow every lookup, for good.
pub const MAX_OBJECTS: usize = 4096;

pub struct Objects {
    slots: Vec<(u32, Object)>,
}

impl Objects {
    /// Object 1 is the display, always, before a client says anything.
    pub fn new() -> Objects {
        Objects { slots: alloc::vec![(1, Object::Display)] }
    }

    pub fn get(&self, id: u32) -> Option<Object> {
        self.slots.iter().find(|(k, _)| *k == id).map(|(_, v)| *v)
    }

    /// Record `id` as `what`; false, recording nothing, when a new id would
    /// pass MAX_OBJECTS, so the request that asked for it makes nothing.
    pub fn put(&mut self, id: u32, what: Object) -> bool {
        let full = self.slots.len() >= MAX_OBJECTS;
        match self.slots.iter_mut().find(|(k, _)| *k == id) {
            Some(slot) => slot.1 = what,
            None if full => return false,
            None => self.slots.push((id, what)),
        }
        true
    }

    pub fn drop_id(&mut self, id: u32) {
        self.slots.retain(|(k, _)| *k != id);
    }
}

impl Default for Objects {
    fn default() -> Objects {
        Objects::new()
    }
}
