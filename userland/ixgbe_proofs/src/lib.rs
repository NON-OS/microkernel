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

//! Host-runnable proofs for the ixgbe (82599, X540, X550) driver capsule.
//!
//! The driver's own files are compiled here through #[path], unmodified, over
//! the libc shim: descriptor parsing and encoding, the rings, the request
//! decode, LINKS decoding, and the reset, link-mode, MAC and queue sequences
//! against a register window in host memory, with a modelled part on a
//! second thread where a handshake needs one. QEMU has no ixgbe device, so
//! these are the only run the driver gets before real hardware.

#[path = "../../capsule_driver_ixgbe/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_ixgbe/src/init/mod.rs"]
pub mod init;
#[path = "../../capsule_driver_ixgbe/src/link/mod.rs"]
pub mod link;
#[path = "../../capsule_driver_ixgbe/src/log/mod.rs"]
pub mod log;
#[path = "../../capsule_driver_ixgbe/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_ixgbe/src/queue/mod.rs"]
pub mod queue;
#[path = "../../capsule_driver_ixgbe/src/regs.rs"]
pub mod regs;
pub mod setup;

#[cfg(test)]
mod tests;
