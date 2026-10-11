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

//! The seat requests beyond a pointer and a keyboard (ops_req.rs), from
//! wayland.xml at wl_seat version 5.

pub const SEAT_GET_TOUCH: u16 = 2;
pub const SEAT_RELEASE: u16 = 3;
pub const POINTER_SET_CURSOR: u16 = 0;
pub const POINTER_RELEASE: u16 = 1;
pub const KEYBOARD_RELEASE: u16 = 0;
pub const TOUCH_RELEASE: u16 = 0;
