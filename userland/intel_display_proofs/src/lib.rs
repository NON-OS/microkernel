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

//! The Intel display engine code, proved against a model of the engine.
//!
//! The model is written from the same PRM registers the engine code cites,
//! and holds the behaviour that matters to a takeover and a flip: writes
//! to a plane's registers change nothing on screen until PLANE_SURF arms
//! them and a vertical blank latches them together; PLANE_SURFLIVE and
//! PIPE_FRMCNT move only at that blank; the scanout reads pixels through
//! the GGTT into IOVA pages. It proves the code against the PRM as this
//! driver reads it, not against silicon; the owner's boot does that.

#[path = "../../capsule_driver_intel_display/src/display/mod.rs"]
pub mod display;
#[path = "../../capsule_driver_intel_display/src/dmc.rs"]
pub mod dmc;
#[path = "../../capsule_driver_intel_display/src/dmc_parse.rs"]
pub mod dmc_parse;
#[path = "../../capsule_driver_intel_display/src/dp/mod.rs"]
pub mod dp;
#[path = "../../capsule_driver_intel_display/src/engine/mod.rs"]
pub mod engine;
#[path = "../../capsule_driver_intel_display/src/family.rs"]
pub mod family;
#[path = "../../capsule_driver_intel_display/src/ids.rs"]
pub mod ids;
#[path = "../../capsule_driver_intel_display/src/ids_parse.rs"]
pub mod ids_parse;

#[cfg(test)]
mod model;
#[cfg(test)]
mod tests;
