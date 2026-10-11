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

//! An object going, and a client that has failed: told of a fatal error,
//! it holds nothing but the display and is served nothing more.

use super::{Object, Objects};

impl Objects {
    pub fn drop_id(&mut self, id: u32) {
        self.slots.retain(|s| s.0 != id);
    }

    pub fn fail(&mut self) {
        self.slots.retain(|s| s.1 == Object::Display);
        self.failed = true;
    }

    pub fn failed(&self) -> bool {
        self.failed
    }
}

impl Default for Objects {
    fn default() -> Objects {
        Objects::new()
    }
}
