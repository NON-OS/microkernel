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

//! Which events an object may be sent. Wayland gives each object the
//! version its global was bound at, and an object a request makes the
//! version of the object the request was sent to; an event a later version
//! added is never sent to an older object, whose client cannot read it.

use super::Objects;

impl Objects {
    /// The request about to be served was sent to `id`, so what it makes
    /// takes that object's version.
    pub fn serving(&mut self, id: u32) {
        self.on = self.slots.iter().find(|s| s.0 == id).map_or(1, |s| s.2);
    }

    /// Whether `id` was made at `version` or later, so an event that
    /// version added may be sent to it.
    pub fn at_least(&self, id: u32, version: u32) -> bool {
        self.slots.iter().any(|s| s.0 == id && s.2 >= version)
    }
}
