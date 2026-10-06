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

//! Host-runnable proofs for the igc (I225/I226) driver capsule.
//!
//! The driver's own files are compiled here through #[path], unmodified, over
//! the libc shim: descriptor parsing and encoding, the rings, the request
//! decode, MDIC, STATUS decoding, and the reset, semaphore, PHY and queue
//! sequences against a register window in host memory, with a modelled part
//! on a second thread where a handshake needs one. There is no QEMU model of
//! this part, so these and the build are all the verification short of the
//! machine itself.

#[path = "../../capsule_driver_igc/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_igc/src/init/mod.rs"]
pub mod init;
#[path = "../../capsule_driver_igc/src/link/mod.rs"]
pub mod link;
#[path = "../../capsule_driver_igc/src/log/mod.rs"]
pub mod log;
#[path = "../../capsule_driver_igc/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_igc/src/queue/mod.rs"]
pub mod queue;
#[path = "../../capsule_driver_igc/src/regs.rs"]
pub mod regs;
pub mod setup;

#[cfg(test)]
mod tests;
