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

//! capsule_linux's own box, mounted: the presets, the `allow:` items, the
//! folder rule, the destination list, the hash, what a terminal run may hold
//! and the report's allowed lines. A policy file is checked by these, never
//! by a copy of them.

#[path = "../../../capsule_linux/src/linux/boxed/allowed.rs"]
pub mod allowed;
#[path = "../../../capsule_linux/src/linux/boxed/dests.rs"]
pub mod dests;
#[path = "../../../capsule_linux/src/linux/boxed/folders.rs"]
pub mod folders;
#[path = "../../../capsule_linux/src/linux/boxed/hash.rs"]
pub mod hash;
#[path = "../../../capsule_linux/src/linux/boxed/items.rs"]
pub mod items;
#[path = "../../../capsule_linux/src/linux/boxed/preset.rs"]
pub mod preset;
#[path = "../../../capsule_linux/src/linux/boxed/terminal_run.rs"]
pub mod terminal_run;
