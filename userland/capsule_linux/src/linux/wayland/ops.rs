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

//! Request and event opcodes, transcribed from the protocol definitions.

#[path = "ops_data.rs"]
pub mod data;
#[path = "ops_decoration.rs"]
pub mod decoration;
#[path = "ops_misc.rs"]
pub mod misc;
#[path = "ops_output.rs"]
pub mod output;
#[path = "ops_req.rs"]
pub mod req;
#[path = "ops_seat.rs"]
pub mod seat;
#[path = "ops_select.rs"]
pub mod select;
#[path = "ops_shell.rs"]
pub mod shell;

pub mod ev {
    pub const CALLBACK_DONE: u16 = 0;
    pub const DISPLAY_ERROR: u16 = 0;
    pub const DISPLAY_DELETE_ID: u16 = 1;
    pub const SHM_FORMAT: u16 = 0;
    pub const BUFFER_RELEASE: u16 = 0;
    pub const XDG_SURFACE_CONFIGURE: u16 = 0;
    pub const TOPLEVEL_CONFIGURE: u16 = 0;
}

/// wl_shm formats. Zero and one are the two every client understands.
pub const FORMAT_ARGB8888: u32 = 0;
pub const FORMAT_XRGB8888: u32 = 1;

/// wl_display.error's code for a request to an object that does not exist.
pub const ERROR_INVALID_OBJECT: u32 = 0;
