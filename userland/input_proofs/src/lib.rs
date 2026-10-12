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

// The PS/2, USB HID and i2c-hid decoders, at the crate root by re-export.
mod drivers;
pub use drivers::{axis, event, packet, parse_report, ps2_keycodes, ps2_mouse_post, sample};
pub use drivers::{damage, gesture, usb_key_repeat, usb_usage_keycode};

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
mod router_desk;

// Every test module, mounted by path from tests_group.rs; the layout tests
// read the keyboard tables the PS/2 and USB HID drivers share, a normal
// dependency because the crate is a plain no_std library.
#[cfg(test)]
mod tests_group;
