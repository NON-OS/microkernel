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

//! The xdg-shell requests and events beyond a toplevel's own (ops_req.rs),
//! from xdg-shell.xml at xdg_wm_base version 2.

pub const BASE_DESTROY: u16 = 0;
pub const BASE_CREATE_POSITIONER: u16 = 1;
pub const SURFACE_GET_POPUP: u16 = 2;
pub const POSITIONER_DESTROY: u16 = 0;
/// set_size to set_offset; set_reactive, set_parent_size and
/// set_parent_configure (version 3) follow.
pub const POSITIONER_SET_FIRST: u16 = 1;
pub const POSITIONER_SET_LAST: u16 = 9;
pub const POPUP_DESTROY: u16 = 0;
pub const POPUP_GRAB: u16 = 1;
pub const POPUP_DONE: u16 = 1;
pub const POPUP_CONFIGURE: u16 = 0;
pub const TOPLEVEL_SET_PARENT: u16 = 1;
pub const TOPLEVEL_SHOW_WINDOW_MENU: u16 = 4;
pub const TOPLEVEL_MOVE: u16 = 5;
pub const TOPLEVEL_RESIZE: u16 = 6;
pub const TOPLEVEL_SET_MAX_SIZE: u16 = 7;
pub const TOPLEVEL_SET_MIN_SIZE: u16 = 8;
pub const TOPLEVEL_SET_MINIMIZED: u16 = 13;
