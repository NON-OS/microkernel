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

//! The virtio-gpu driver, proved without the device.
//!
//! The `#[path]` includes pull in the shipping driver source, so these tests
//! run the code that boots. Its register and DMA regions are bounds-checked
//! views (`nonos_virtio::Mmio`), so windows in host memory stand in for the
//! common, notify and device capability regions and for the queue's grant.
//! The parts that talk to the kernel (discovery's list, the broker, the
//! receive loop) stay out, but for the DMA grants, the PCI command and the
//! release, which the shim makes and keeps; the clock counts each question
//! a wait asks as a millisecond, and a test may let more pass.
//!
//! Held to the specification here: the bring-up on the shared transport
//! (the status handshake, the features taken against those offered, where
//! the rings are, the doorbell's bound, Bus Master only once the device is
//! seen reset and never on a refusal), the control queue (each command a
//! two-descriptor chain, its answer read from the descriptor the part
//! named, a command that times out ending in a reset, never a stale chain
//! run with new bytes), and the wire bytes of each command, checked by a
//! part on its own thread that consumes the ring the way a device does.
//!
//! And the driver on the display wire (G-77): a host model answers the ring
//! on the driver's own thread through the yield each wait makes, keeps the
//! host's side (resources, their pixels and backings, what each scanout
//! shows), and can be cued to stop answering, or to kill the driver, at any
//! command, under the wire server and its sweep; and the recovery from a
//! stall, every output shown again on its own surface, or to `Down` with
//! nothing released.

extern crate alloc;

pub mod clock;
#[path = "../../capsule_driver_virtio_gpu/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_virtio_gpu/src/device/mod.rs"]
pub mod device;
pub mod discover;
#[path = "../../capsule_driver_virtio_gpu/src/display/mod.rs"]
pub mod display;
#[path = "../../capsule_driver_virtio_gpu/src/driver.rs"]
pub mod driver;
#[path = "../../capsule_driver_virtio_gpu/src/init/mod.rs"]
pub mod init;
pub mod parts;
#[path = "../../capsule_driver_virtio_gpu/src/regs.rs"]
pub mod regs;
pub mod server;
pub mod setup;
#[path = "../../capsule_driver_virtio_gpu/src/state/mod.rs"]
pub mod state;
#[path = "../../capsule_driver_virtio_gpu/src/virgl/mod.rs"]
mod virgl;

#[cfg(test)]
mod tests;
