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

//! zxdg_output_manager_v1 and zxdg_output_v1 (xdg-output-unstable-v1.xml,
//! version 2), and wp_viewporter and wp_viewport (viewporter.xml).

pub const XDG_OUTPUT_MANAGER_DESTROY: u16 = 0;
pub const XDG_OUTPUT_MANAGER_GET: u16 = 1;
pub const XDG_OUTPUT_DESTROY: u16 = 0;
pub const XDG_OUTPUT_LOGICAL_POSITION: u16 = 0;
pub const XDG_OUTPUT_LOGICAL_SIZE: u16 = 1;
pub const XDG_OUTPUT_DONE: u16 = 2;
pub const XDG_OUTPUT_NAME: u16 = 3;
pub const XDG_OUTPUT_DESCRIPTION: u16 = 4;
pub const VIEWPORTER_DESTROY: u16 = 0;
pub const VIEWPORTER_GET_VIEWPORT: u16 = 1;
pub const VIEWPORT_DESTROY: u16 = 0;
pub const VIEWPORT_SET_SOURCE: u16 = 1;
pub const VIEWPORT_SET_DESTINATION: u16 = 2;
