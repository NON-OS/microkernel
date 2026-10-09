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

//! The window manager's `window` module at the path its own files reach it by,
//! `crate::window`, with the same re-exports as capsule_wm/src/window/mod.rs.
//! Only the table is assembled here (see window_table.rs).

#[path = "../../capsule_wm/src/window/full_screen.rs"]
pub mod full_screen;

#[path = "../../capsule_wm/src/window/kind.rs"]
pub mod kind;

#[path = "../../capsule_wm/src/window/reopen.rs"]
pub mod reopen;

#[path = "window_table.rs"]
pub mod table;

// The capsule's own layout: window/mod.rs holds window/window.rs as `window`.
#[allow(clippy::module_inception)]
#[path = "../../capsule_wm/src/window/window.rs"]
pub mod window;

pub use kind::{from_u32 as kind_from_u32, Kind};
pub use table::WindowTable;
pub use window::{Visibility, Window};
