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

//! Host proofs for the window manager's geometry. The `#[path]` includes pull
//! in the real production source so the tests pin the shipping hit-testing and
//! clamping used by click-to-raise and window placement.

// The geometry module includes rect and constrain once; the tests reach them
// at the crate root through these names.
pub use geometry::{constrain, rect};

// The stack, the hit test and click to focus, under the `crate::` paths their
// files name each other by: geometry, window, z_order and focus.
#[path = "../../capsule_wm/src/geometry/mod.rs"]
pub mod geometry;

pub mod window;

#[path = "../../capsule_wm/src/z_order/mod.rs"]
pub mod z_order;

#[path = "../../capsule_wm/src/focus/mod.rs"]
pub mod focus;

// The wire every request arrives on: the header decode the loop runs before
// dispatch, and the encoders a refusal is answered with.
#[path = "../../capsule_wm/src/protocol/mod.rs"]
pub mod protocol;

// The request handlers that read a body without a kernel call.
pub mod server;

// Whether the compositor is owed the stack again after a lost focus_set.
#[path = "../../capsule_wm/src/state/restack.rs"]
pub mod restack;

#[cfg(test)]
mod restack_tests;

#[cfg(test)]
mod full_screen_tests;
#[cfg(test)]
mod geometry_tests;
#[cfg(test)]
mod hand_off_tests;
#[cfg(test)]
mod notify_send_tests;
#[cfg(test)]
mod parse_tests;
#[cfg(test)]
mod press_tests;
#[cfg(test)]
mod reopen_tests;
#[cfg(test)]
mod window_share_tests;
