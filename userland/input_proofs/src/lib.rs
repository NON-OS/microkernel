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

//! Host proofs for the input decoders. Each `#[path]` include pulls in the real
//! driver source so the tests pin production decode logic, not a copy. The
//! module names match what the included files reach for via `super::`.

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

// The i2c-hid mouse sample type the HID mouse decode returns.
#[path = "../../capsule_driver_i2c_hid/src/input/sample.rs"]
pub mod sample;
// The sample types at `crate::input::sample`, where the HID mouse decode names
// them.
pub mod input {
    pub use crate::sample;
}

// The HID report-descriptor parser and the absolute-touch decoder, under
// the path the driver's own files name them by, `crate::hid`.
pub mod hid;

// The touchpad gesture state machine (tap, move, two-finger scroll).
#[path = "../../capsule_driver_i2c_hid/src/input/gesture/mod.rs"]
pub mod gesture;

// The compositor's damage accumulator, whose coalescing must never drop a
// damaged pixel or the screen tears.
#[path = "../../compositor/src/state/damage/mod.rs"]
pub mod damage;

// The input router's routing, whole (router.rs), and its route tree.
mod router;
pub use router::*;
#[path = "../../capsule_input_router/src/route/mod.rs"]
pub mod route;

#[cfg(test)]
mod router_desk;
#[cfg(test)]
mod router_tests;
#[cfg(test)]
mod tests;
