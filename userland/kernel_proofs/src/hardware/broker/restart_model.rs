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

//! The restart model's state: the live capsule, the pid the kernel records
//! for it, who took the button, who holds the Global Lock.

use super::super::acpi::leave_rule::lock_held;

/// `recorded` is the capsule's pid as the kernel records it: it keeps a
/// dead pid until the supervisor looks.
#[derive(Default)]
pub struct Model {
    pub alive: u32,
    pub recorded: u32,
    pub taken: u32,
    pub lock: u32,
}

pub fn lock(m: &mut Model, held: bool) {
    lock_held(m.alive, held);
    m.lock = if held { m.alive } else { 0 };
}
