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

//! zwp_primary_selection_device_manager_v1 and its device and source
//! (primary-selection-unstable-v1.xml), and wp_cursor_shape_manager_v1 and
//! its device (cursor-shape-v1.xml).

pub const PRIMARY_MANAGER_CREATE_SOURCE: u16 = 0;
pub const PRIMARY_MANAGER_GET_DEVICE: u16 = 1;
pub const PRIMARY_MANAGER_DESTROY: u16 = 2;
pub const PRIMARY_DEVICE_SET_SELECTION: u16 = 0;
pub const PRIMARY_DEVICE_DESTROY: u16 = 1;
pub const PRIMARY_SOURCE_OFFER: u16 = 0;
pub const PRIMARY_SOURCE_DESTROY: u16 = 1;
pub const PRIMARY_SOURCE_CANCELLED: u16 = 1;
pub const CURSOR_MANAGER_DESTROY: u16 = 0;
pub const CURSOR_MANAGER_GET_POINTER: u16 = 1;
pub const CURSOR_DEVICE_DESTROY: u16 = 0;
pub const CURSOR_DEVICE_SET_SHAPE: u16 = 1;
