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

//! Intel HD Audio controller bring-up, proved without a controller.
//!
//! The `#[path]` includes below pull in the shipping driver source, so these
//! tests run the code that boots. `nonos_devmodel::run` puts a device on a
//! thread to drive the other end of the rings, and `libc_shim` stands in for
//! the broker, recording every call. What is proved is what the HD Audio
//! specification fixes in writing, and the order the teardown keeps; timing
//! is not modelled, and the serving loop that waits on IPC is not mounted.

extern crate alloc;

#[path = "../../capsule_driver_hda/src/regs/mod.rs"]
pub mod regs;

/// The host's clock in place of the capsule's (`capsule_driver_hda/src/clock.rs`),
/// so every bounded wait in the controller source ends on the host too.
pub mod clock;

#[path = "../../capsule_driver_hda/src/constants/mod.rs"]
pub mod constants;

#[path = "../../capsule_driver_hda/src/error/mod.rs"]
pub mod error;

/// The request wire the serving loop reads with: the header decode every
/// frame passes before dispatch, and the encoders a refusal is answered with.
#[path = "../../capsule_driver_hda/src/protocol/mod.rs"]
pub mod protocol;

#[cfg(test)]
pub mod controller;

#[cfg(test)]
#[path = "../../capsule_driver_hda/src/audio/mod.rs"]
mod audio;
#[cfg(test)]
#[path = "../../capsule_driver_hda/src/discover/mod.rs"]
mod discover;
#[cfg(test)]
#[path = "../../capsule_driver_hda/src/handles/mod.rs"]
mod handles;
#[cfg(test)]
mod server;
#[cfg(test)]
#[path = "../../capsule_driver_hda/src/setup/mod.rs"]
mod setup;

#[cfg(test)]
mod model;
#[cfg(test)]
mod proofs;
#[cfg(test)]
mod request_refusal_tests;
#[cfg(test)]
mod sim;
