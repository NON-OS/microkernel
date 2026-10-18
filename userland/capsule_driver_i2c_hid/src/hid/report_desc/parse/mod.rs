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

//! A report descriptor walked into the touchpad's and the mouse's field
//! maps. The item reader, the globals with Push and Pop, and the offset
//! each report ID has reached are usb_hid's parser, shared as its source:
//! one walker for both HID drivers. The bounds, the locals and what is
//! collected from the walk are this driver's.

#[path = "../../../../../capsule_driver_usb_hid/src/descriptors/report/cursors.rs"]
mod cursors;
#[path = "../../../../../capsule_driver_usb_hid/src/descriptors/report/global_state.rs"]
mod global_state;
#[path = "../../../../../capsule_driver_usb_hid/src/descriptors/report/globals.rs"]
mod globals;
#[path = "../../../../../capsule_driver_usb_hid/src/descriptors/report/item.rs"]
mod item;

mod assign;
mod feature;
mod field_at;
mod fingers;
mod hand;
mod limits;
mod locals;
mod long;
mod main_item;
mod mouse;
mod take;
mod touch;
mod walk;

pub use fingers::{parse_fingers, Surface};
pub use mouse::parse_mouse;
pub use touch::parse;
