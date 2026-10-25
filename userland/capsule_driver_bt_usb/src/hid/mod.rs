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

mod keys;
mod keys_press;
mod modifier;

// A Bluetooth keyboard's keys must reach the input ring exactly as a USB
// keyboard's do: the same layout, the same codes, the same repeat. So the
// USB HID driver's own files are compiled here, not copied: one change
// there changes both.
#[allow(dead_code)]
#[path = "../../../capsule_driver_usb_hid/src/hid/active.rs"]
mod active;
#[allow(dead_code)]
#[path = "../../../capsule_driver_usb_hid/src/hid/key_event.rs"]
mod key_event;
#[allow(dead_code)]
#[path = "../../../capsule_driver_usb_hid/src/hid/keymap/mod.rs"]
mod keymap;
#[path = "../../../capsule_driver_usb_hid/src/hid/key_post.rs"]
mod key_post;
#[path = "../../../capsule_driver_usb_hid/src/hid/post_key.rs"]
mod post_key;
#[allow(dead_code)]
#[path = "../../../capsule_driver_usb_hid/src/hid/post_wire.rs"]
mod post_wire;
#[allow(dead_code)]
#[path = "../../../capsule_driver_usb_hid/src/hid/keyboard/repeat/mod.rs"]
mod repeat;
#[path = "../../../capsule_driver_usb_hid/src/hid/usage_keycode/mod.rs"]
mod usage_keycode;

pub use keys::Keys;
