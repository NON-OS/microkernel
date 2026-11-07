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

//! Each capsule init's plan spawns, once a boot: a later walk of the plan
//! (`late`) spawns only what was not tried. One that ended is the
//! supervisor's to restart, never the plan's. Pure: `kernel_proofs`
//! mounts it.

use alloc::vec::Vec;

pub(super) struct Tried {
    names: Vec<&'static str>,
}

impl Tried {
    pub(super) const fn new() -> Self {
        Tried { names: Vec::new() }
    }

    /// True the first time `name` is asked for.
    pub(super) fn first(&mut self, name: &'static str) -> bool {
        if self.names.contains(&name) {
            return false;
        }
        self.names.push(name);
        true
    }
}
