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

// i2c-hid touchpad report decoder and its sample type.
#[path = "../../capsule_driver_i2c_hid/src/input/parse_report.rs"]
pub mod parse_report;
#[path = "../../capsule_driver_i2c_hid/src/input/sample.rs"]
pub mod sample;
// The sample types at `crate::input::sample`, where the HID mouse decode names
// them.
pub mod input {
    pub use crate::sample;
}

/*
 * The HID report-descriptor parser and the absolute-touch decoder, under the
 * path the driver's own files name them by, `crate::hid`.
 */
pub mod hid;

// The touchpad gesture state machine (tap, move, two-finger scroll).
#[path = "../../capsule_driver_i2c_hid/src/input/gesture/mod.rs"]
pub mod gesture;

// The compositor's damage accumulator, whose coalescing must never drop a
// damaged pixel or the screen tears.
#[path = "../../compositor/src/state/damage.rs"]
pub mod damage;

// The input router's press grab: the frame a dragged window's motion arrives in.
pub use state::press as router_press;

// The input router's request wire: the header decode every frame in its inbox
// passes before dispatch, and the encoders a refusal is answered with.
pub mod router_protocol;

// The router's reserved chord, which no window is handed.
pub use route::chord as router_chord;

/*
 * The input router's routing, whole: its state and route trees under the
 * `crate::` paths they name each other by, its delivery frame, and its peers
 * (window manager, compositor, service lookup) answering from a desk the
 * tests lay out (`clients`). nonos_libc is the shim beside this crate, whose
 * sends land in an outbox the tests read back.
 */
#[path = "../../capsule_input_router/src/state/mod.rs"]
pub mod state;

#[path = "../../capsule_input_router/src/route/mod.rs"]
pub mod route;

pub mod clients;
pub mod protocol;

// Which process a key press goes to, by what the window manager answered.
pub use route::key_target as router_key_target;

#[cfg(test)]
mod chord_tests;
#[cfg(test)]
mod damage_tests;
#[cfg(test)]
mod gesture_click_tests;
#[cfg(test)]
mod gesture_tests;
#[cfg(test)]
mod held_keys_tests;
#[cfg(test)]
mod hid_touchpad_tests;
#[cfg(test)]
mod key_target_tests;
#[cfg(test)]
mod press_frame_tests;
#[cfg(test)]
mod ps2_tests;
#[cfg(test)]
mod router_desk;
#[cfg(test)]
mod router_hover_tests;
#[cfg(test)]
mod router_key_tests;
#[cfg(test)]
mod router_parse_tests;
#[cfg(test)]
mod router_press_tests;
#[cfg(test)]
mod router_route_tests;
#[cfg(test)]
mod router_system_key_tests;
#[cfg(test)]
mod touchpad_tests;
#[cfg(test)]
mod usb_keycode_tests;
#[cfg(test)]
mod usb_repeat_tests;

// Keyboard layout tables shared by the PS/2 and USB HID drivers; a normal
// dependency because the crate is a plain no_std library.
#[cfg(test)]
mod layout_tests;
