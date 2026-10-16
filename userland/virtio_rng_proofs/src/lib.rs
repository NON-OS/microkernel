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

//! The virtio-rng driver, proved against a register window and DMA memory
//! rather than against a device.
//!
//! Each `#[path]` below pulls in the shipping driver source, so these tests
//! run the code that boots, not a copy of it. The parts that call the
//! kernel (discovery, the broker, the IPC server) stay out; the clock is
//! the one stand-in, counting each question a wait asks as a millisecond.
//!
//! Two things are held here. The handshake, which the virtio specification
//! fixes, not the device: the status byte driven reset, ACKNOWLEDGE,
//! DRIVER, FEATURES_OK, DRIVER_OK in that order, the legacy ring size taken
//! as the device offers it or refused. And the request queue: one request
//! in flight at most, each completion read from its own element, a request
//! that timed out never served and never posted over, so no entropy byte
//! goes out twice, whatever the device does late.

pub mod clock;
#[path = "../../capsule_driver_virtio_rng/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_virtio_rng/src/fill.rs"]
pub mod fill;
#[path = "../../capsule_driver_virtio_rng/src/init/mod.rs"]
pub mod init;
#[path = "../../capsule_driver_virtio_rng/src/setup/modern/queue.rs"]
pub mod modern_queue;
#[path = "../../capsule_driver_virtio_rng/src/queue/mod.rs"]
pub mod queue;
pub mod regs;
pub mod transport;

#[cfg(test)]
mod tests;
