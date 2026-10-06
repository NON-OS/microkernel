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

//! The window table, file for file as capsule_wm/src/window/table/mod.rs lists
//! it, less `remove_one_dead`: that one asks the kernel whether a process is
//! alive, and nothing here runs a kernel to ask.

#[path = "../../capsule_wm/src/window/table/find.rs"]
mod find;
#[path = "../../capsule_wm/src/window/table/find_mut.rs"]
mod find_mut;
#[path = "../../capsule_wm/src/window/table/held_by.rs"]
mod held_by;
#[path = "../../capsule_wm/src/window/table/insert.rs"]
mod insert;
#[path = "../../capsule_wm/src/window/table/new.rs"]
mod new;
#[path = "../../capsule_wm/src/window/table/remove.rs"]
mod remove;
#[path = "../../capsule_wm/src/window/table/types.rs"]
mod types;
#[path = "../../capsule_wm/src/window/table/windows.rs"]
mod windows;

pub use types::{WindowTable, MAX_WINDOWS, PER_OWNER};
