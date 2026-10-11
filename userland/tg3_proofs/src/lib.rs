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

//! Host-runnable proofs for the tg3 (Broadcom 57765 class) driver capsule.
//!
//! The driver's own files are compiled here through #[path], unmodified,
//! over the libc shim: the board-config decode, the rings and the entries
//! a hostile part could write back, the mode words held to Linux's, the
//! configuration writes held to the kernel broker's own rule (its
//! quirk_bits module is mounted beside them), and the PHY and the whole
//! bring-up against a register window in host memory with a modelled PHY
//! on a second thread. QEMU has no tg3, so these are the only run the
//! driver gets before real hardware.

#[path = "../../capsule_driver_tg3/src/chip/mod.rs"]
pub mod chip;
#[path = "../../capsule_driver_tg3/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_tg3/src/init/mod.rs"]
pub mod init;
#[path = "../../capsule_driver_tg3/src/link/mod.rs"]
pub mod link;
#[path = "../../capsule_driver_tg3/src/log/mod.rs"]
pub mod log;
#[path = "../../capsule_driver_tg3/src/phy/mod.rs"]
pub mod phy;
#[path = "../../capsule_driver_tg3/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_tg3/src/queue/mod.rs"]
pub mod queue;
#[path = "../../../src/hardware/broker/pci/quirk_bits/mod.rs"]
pub mod quirk_bits;
#[path = "../../capsule_driver_tg3/src/regs.rs"]
pub mod regs;
pub mod setup;

#[cfg(test)]
mod tests;
