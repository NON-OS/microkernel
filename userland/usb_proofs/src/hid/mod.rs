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

//! The driver's report decoders and the per-interface state they feed,
//! under the path its own files name them by, `crate::hid`. The posts land
//! in the libc shim's list; only the layout policy, which asks the policy
//! store over IPC, is replaced, by `active` here.

pub mod active;
#[path = "../../../capsule_driver_usb_hid/src/hid/button_changes.rs"]
pub mod button_changes;
#[path = "../../../capsule_driver_usb_hid/src/hid/device.rs"]
pub mod device;
#[path = "../../../capsule_driver_usb_hid/src/hid/key_event.rs"]
pub mod key_event;
pub mod keyboard;
#[path = "../../../capsule_driver_usb_hid/src/hid/keymap/mod.rs"]
pub mod keymap;
#[path = "../../../capsule_driver_usb_hid/src/hid/mouse.rs"]
pub mod mouse;
#[path = "../../../capsule_driver_usb_hid/src/hid/mouse_event.rs"]
pub mod mouse_event;
#[path = "../../../capsule_driver_usb_hid/src/hid/mouse_report.rs"]
pub mod mouse_report;
#[path = "../../../capsule_driver_usb_hid/src/hid/post_failures.rs"]
pub mod post_failures;
#[path = "../../../capsule_driver_usb_hid/src/hid/post_key.rs"]
pub mod post_key;
#[path = "../../../capsule_driver_usb_hid/src/hid/post_mouse.rs"]
pub mod post_mouse;
#[path = "../../../capsule_driver_usb_hid/src/hid/post_wire.rs"]
pub mod post_wire;
pub mod report;
#[path = "../../../capsule_driver_usb_hid/src/hid/tablet.rs"]
pub mod tablet;
#[path = "../../../capsule_driver_usb_hid/src/hid/usage_keycode/mod.rs"]
pub mod usage_keycode;

pub use device::Device;
pub use keyboard::Keyboard;
pub use mouse::Mouse;
pub use report::ReportDevice;
pub use tablet::Tablet;
