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

//! Request opcodes, transcribed from the protocol definitions.

pub const DISPLAY_SYNC: u16 = 0;
pub const DISPLAY_GET_REGISTRY: u16 = 1;
pub const REGISTRY_BIND: u16 = 0;

pub const COMPOSITOR_CREATE_SURFACE: u16 = 0;
pub const COMPOSITOR_CREATE_REGION: u16 = 1;
pub const REGION_DESTROY: u16 = 0;
pub const REGION_ADD: u16 = 1;
pub const REGION_SUBTRACT: u16 = 2;
pub const SUBCOMPOSITOR_DESTROY: u16 = 0;
pub const SUBCOMPOSITOR_GET_SUBSURFACE: u16 = 1;
pub const SUBSURFACE_DESTROY: u16 = 0;
pub const SUBSURFACE_SET_POSITION: u16 = 1;
pub const SUBSURFACE_PLACE_ABOVE: u16 = 2;
pub const SUBSURFACE_PLACE_BELOW: u16 = 3;
pub const SUBSURFACE_SET_SYNC: u16 = 4;
pub const SUBSURFACE_SET_DESYNC: u16 = 5;

pub const SHM_CREATE_POOL: u16 = 0;
pub const POOL_CREATE_BUFFER: u16 = 0;
pub const POOL_DESTROY: u16 = 1;
pub const POOL_RESIZE: u16 = 2;
pub const BUFFER_DESTROY: u16 = 0;

pub const SURFACE_DESTROY: u16 = 0;
pub const SURFACE_ATTACH: u16 = 1;
pub const SURFACE_DAMAGE: u16 = 2;
pub const SURFACE_FRAME: u16 = 3;
pub const SURFACE_SET_OPAQUE_REGION: u16 = 4;
pub const SURFACE_SET_INPUT_REGION: u16 = 5;
pub const SURFACE_COMMIT: u16 = 6;
pub const SURFACE_SET_BUFFER_TRANSFORM: u16 = 7;
pub const SURFACE_SET_BUFFER_SCALE: u16 = 8;
pub const SURFACE_DAMAGE_BUFFER: u16 = 9;

pub const XDG_BASE_GET_SURFACE: u16 = 2;
pub const XDG_BASE_PONG: u16 = 3;
pub const XDG_SURFACE_DESTROY: u16 = 0;
pub const XDG_SURFACE_GET_TOPLEVEL: u16 = 1;
pub const XDG_SURFACE_SET_GEOMETRY: u16 = 3;
pub const XDG_SURFACE_ACK_CONFIGURE: u16 = 4;
pub const TOPLEVEL_DESTROY: u16 = 0;
pub const TOPLEVEL_SET_TITLE: u16 = 2;
pub const TOPLEVEL_SET_APP_ID: u16 = 3;
pub const TOPLEVEL_SET_MAXIMIZED: u16 = 9;
pub const TOPLEVEL_UNSET_MAXIMIZED: u16 = 10;
pub const TOPLEVEL_SET_FULLSCREEN: u16 = 11;
pub const TOPLEVEL_UNSET_FULLSCREEN: u16 = 12;

pub const SEAT_GET_POINTER: u16 = 0;
pub const SEAT_GET_KEYBOARD: u16 = 1;
