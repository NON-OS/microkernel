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

//! Which names a capsule loaded at run time may declare (K18): the kernel's
//! own rule and its own reserved tables, by path.

#[path = "../../../../src/kernel_core/process_spawn/capsule_spawn/from_vfs/load/baked_apps.rs"]
pub mod baked_apps;
#[path = "../../../../src/kernel_core/process_spawn/capsule_spawn/from_vfs/load/baked_names.rs"]
pub mod baked_names;
#[cfg(test)]
mod family_tests;
#[path = "../../../../src/kernel_core/process_spawn/capsule_spawn/from_vfs/load/name_rule.rs"]
pub mod rule;
#[cfg(test)]
mod tests;
