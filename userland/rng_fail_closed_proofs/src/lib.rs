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

//! The kernel's files, mounted where the kernel has them, so their `crate::`
//! and `super::` paths resolve as they do there. Only the seeded RNG, the
//! CPU's random instruction, the time counter and the serial line stand in
//! (`stand_in`).

#![cfg(test)]
#![allow(dead_code)]

include!("mount.rs");
mod tests;
