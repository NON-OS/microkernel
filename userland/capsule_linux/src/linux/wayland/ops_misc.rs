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

//! xdg_activation_v1 and its token (xdg-activation-v1.xml), and
//! wp_single_pixel_buffer_manager_v1 (single-pixel-buffer-v1.xml).

pub const ACTIVATION_DESTROY: u16 = 0;
pub const ACTIVATION_GET_TOKEN: u16 = 1;
pub const ACTIVATION_ACTIVATE: u16 = 2;
/// set_serial, set_app_id, set_surface.
pub const TOKEN_SET_FIRST: u16 = 0;
pub const TOKEN_SET_LAST: u16 = 2;
pub const TOKEN_COMMIT: u16 = 3;
pub const TOKEN_DESTROY: u16 = 4;
pub const TOKEN_DONE: u16 = 0;
pub const SINGLE_PIXEL_DESTROY: u16 = 0;
pub const SINGLE_PIXEL_CREATE_BUFFER: u16 = 1;
