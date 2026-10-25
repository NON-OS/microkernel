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

//! Dropping what the runtime keeps about an object when the object is
//! removed (Unload, or a method's own objects at its end). The slot may be
//! reused for a new object, which must not inherit any of it: a value, a
//! region, a holder that would make a new mutex look taken, signals a new
//! event never had, a notification meant for the device that was there,
//! or a _REG mark.

use super::Runtime;
use crate::namespace::Id;

impl Runtime {
    pub fn forget(&mut self, id: Id) {
        *self.value(id) = None;
        self.regions.remove(&id);
        self.buffer_fields.remove(&id);
        self.mutexes.remove(&id);
        self.events.remove(&id);
        self.notifications.retain(|(n, _)| *n != id);
        self.announced.retain(|(s, _)| *s != id);
    }
}
