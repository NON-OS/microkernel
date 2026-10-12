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

//! The display wire's server core, proved without a kernel or a device.
//!
//! The `#[path]` includes pull in the shipping crate at the module names
//! its `super::` paths use. Requests are built as the compositor builds
//! them and answered as a driver answers them.

#[path = "../../nonos_gfx_serve/src/copy.rs"]
pub mod copy;
#[path = "../../nonos_gfx_serve/src/display.rs"]
pub mod display;
#[path = "../../nonos_gfx_serve/src/flush.rs"]
pub mod flush;
#[path = "../../nonos_gfx_serve/src/handle.rs"]
pub mod handle;
#[path = "../../nonos_gfx_serve/src/mode_check.rs"]
pub mod mode_check;
#[path = "../../nonos_gfx_serve/src/mode_set.rs"]
pub mod mode_set;
#[path = "../../nonos_gfx_serve/src/platform.rs"]
pub mod platform;
#[path = "../../nonos_gfx_serve/src/primary.rs"]
pub mod primary;
#[path = "../../nonos_gfx_serve/src/server.rs"]
pub mod server;
#[path = "../../nonos_gfx_serve/src/slot.rs"]
pub mod slot;
#[path = "../../nonos_gfx_serve/src/state.rs"]
pub mod state;
#[path = "../../nonos_gfx_serve/src/surface.rs"]
pub mod surface;

#[cfg(test)]
mod model;
#[cfg(test)]
mod tests;
