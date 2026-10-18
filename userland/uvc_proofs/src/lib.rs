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
//! Proofs for the USB Video Class core, run on the host against the
//! library itself, and for driver.uvc0's pure parts, mounted from the
//! capsule at the paths it names them by: the camera wire, the capture
//! gate, the indicator notice, the format listing and a batch's payloads.

extern crate alloc;

#[path = "../../capsule_driver_uvc/src/batch.rs"]
pub mod batch;
#[path = "../../capsule_driver_uvc/src/gate.rs"]
pub mod gate;
#[path = "../../capsule_driver_uvc/src/indicator.rs"]
pub mod indicator;
#[path = "../../capsule_driver_uvc/src/listing.rs"]
pub mod listing;
#[path = "../../capsule_driver_uvc/src/wire.rs"]
pub mod wire;

#[cfg(test)]
mod bound_tests;
#[cfg(test)]
mod camera_tests;
#[cfg(test)]
mod frame_tests;
#[cfg(test)]
mod gate_tests;
#[cfg(test)]
mod hostile_tests;
#[cfg(test)]
mod indicator_tests;
#[cfg(test)]
mod listing_tests;
#[cfg(test)]
mod payload_sweep;
#[cfg(test)]
mod payload_sweep_tests;
#[cfg(test)]
mod probe_tests;
#[cfg(test)]
mod webcam;
