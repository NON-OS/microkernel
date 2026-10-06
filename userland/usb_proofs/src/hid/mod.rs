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

//! The driver's report decoders, under the path its own files name them by,
//! `crate::hid`. The parts that post input events or read the layout policy
//! make system calls and stay out; every decision about report bytes lives
//! in these files.

#[path = "../../../capsule_driver_usb_hid/src/hid/button_changes.rs"]
pub mod button_changes;
pub mod keyboard;
#[path = "../../../capsule_driver_usb_hid/src/hid/mouse_event.rs"]
pub mod mouse_event;
#[path = "../../../capsule_driver_usb_hid/src/hid/mouse_report.rs"]
pub mod mouse_report;
#[path = "../../../capsule_driver_usb_hid/src/hid/tablet_report.rs"]
pub mod tablet_report;
