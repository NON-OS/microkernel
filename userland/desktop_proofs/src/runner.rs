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

//! app_skeleton's runner pieces the pointer path goes through. Their items
//! are visible to `runner` only, as in app_skeleton, so the tests that drive
//! them live under it too, and nothing outside the tests reaches them.

// The runtime's bar and dock, taken from app_skeleton itself rather than by
// path: the file carries its own copy of the brand's scale rule, and the
// shell's scale included below carries another, so the two can only meet in
// one crate as two crates.
pub use nonos_app_skeleton::runner::chrome;

// How small any app's window may be made; the drag takes its minimum from it.
#[path = "../../app_skeleton/src/runner/min_size.rs"]
pub mod min_size;

#[cfg(test)]
#[path = "../../app_skeleton/src/runner/dispatch.rs"]
mod dispatch;

#[cfg(test)]
#[path = "../../app_skeleton/src/runner/drag.rs"]
mod drag;

#[cfg(test)]
#[path = "../../app_skeleton/src/runner/press_part.rs"]
mod press_part;

#[cfg(test)]
#[path = "desk.rs"]
mod desk;

// An app's ask for full screen, followed the way the green button goes.
#[cfg(test)]
pub use nonos_app_skeleton::runner::full_screen_ask;

#[cfg(test)]
#[path = "full_screen_desk_tests.rs"]
mod full_screen_desk_tests;

#[cfg(test)]
#[path = "one_press_tests.rs"]
mod one_press_tests;

#[cfg(test)]
#[path = "../../app_skeleton/src/runner/teardown_steps.rs"]
mod teardown_steps;

#[cfg(test)]
#[path = "../../app_skeleton/src/runner/no_window.rs"]
mod no_window;

#[cfg(test)]
#[path = "teardown_tests.rs"]
mod teardown_tests;

#[cfg(test)]
#[path = "restack_desk_tests.rs"]
mod restack_desk_tests;

#[cfg(test)]
#[path = "resize_desk_tests.rs"]
mod resize_desk_tests;

#[cfg(test)]
#[path = "drag_tests.rs"]
mod drag_tests;

#[cfg(test)]
#[path = "delivery_tests.rs"]
mod delivery_tests;

#[cfg(test)]
#[path = "frame_scale_tests.rs"]
mod frame_scale_tests;

#[cfg(test)]
#[path = "../../app_skeleton/src/runner/open_peers.rs"]
mod open_peers;

#[cfg(test)]
#[path = "open_peers_tests.rs"]
mod open_peers_tests;

// The rounded bottom corners a partial repaint redraws over its rows.
#[cfg(test)]
#[path = "../../app_skeleton/src/runner/finish_band.rs"]
mod finish_band;

#[cfg(test)]
#[path = "finish_band_tests.rs"]
mod finish_band_tests;

// A partial repaint drawn off the surface and written back whole.
#[cfg(test)]
#[path = "../../app_skeleton/src/runner/off_screen.rs"]
mod off_screen;

#[cfg(test)]
#[path = "off_screen_tests.rs"]
mod off_screen_tests;
