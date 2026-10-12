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
//! The driver decoders the tests pin, each its real source by `#[path]`; the
//! files reach one another as siblings, by `super::`.

// PS/2 mouse packet decoder and the event type it returns.
#[path = "../../capsule_driver_ps2_input/src/mouse/axis.rs"]
pub mod axis;
#[path = "../../capsule_driver_ps2_input/src/mouse/event.rs"]
pub mod event;
#[path = "../../capsule_driver_ps2_input/src/mouse/packet.rs"]
pub mod packet;
// The PS/2 keycode table apps read for the keys that are not characters, and
// the USB HID driver's map onto it.
#[path = "../../capsule_driver_ps2_input/src/keymap/set1/keycodes.rs"]
pub mod ps2_keycodes;
#[path = "../../capsule_driver_usb_hid/src/hid/usage_keycode/mod.rs"]
pub mod usb_usage_keycode;
// The USB HID driver's key repeat, which a PS/2 keyboard does itself.
#[path = "../../capsule_driver_usb_hid/src/hid/keyboard/repeat/mod.rs"]
pub mod usb_key_repeat;
// What the PS/2 driver posts to the input ring for one decoded packet.
#[path = "../../capsule_driver_ps2_input/src/mouse/post.rs"]
pub mod ps2_mouse_post;

// i2c-hid touchpad report decoder and its sample type.
#[path = "../../capsule_driver_i2c_hid/src/input/parse_report.rs"]
pub mod parse_report;
#[path = "../../capsule_driver_i2c_hid/src/input/sample.rs"]
pub mod sample;

// The touchpad gesture state machine (tap, move, two-finger scroll).
#[path = "../../capsule_driver_i2c_hid/src/input/gesture/mod.rs"]
pub mod gesture;

// The compositor's damage accumulator, whose coalescing must never drop a
// damaged pixel or the screen tears.
#[path = "../../compositor/src/state/damage.rs"]
pub mod damage;
