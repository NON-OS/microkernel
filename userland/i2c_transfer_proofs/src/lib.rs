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

//! The I2C transfer engine, proved against a controller that answers.
//!
//! `i2c_pci_proofs` runs the bring-up against a register window in memory,
//! which is all a bring-up needs. A transfer needs more: the data register is
//! a FIFO whose reads pop and whose writes push, and reading the abort-clear
//! register is what clears the abort. Memory cannot see a read.
//!
//! So this crate swaps one file. Every `#[path]` below is the shipping driver;
//! `regs` alone is not (nor `gpio0`'s look, which no gpio0 answers here),
//! and it forwards each access to a DesignWare core modelled from the
//! databook (`nonos_i2cmodel`), with a HID touchpad on the
//! bus behind it. Discovery, the engine, the probe and the IPC handler are
//! then run exactly as they ship, discovery over a device list the shim
//! answers from records the tests build, and what the proofs read is the
//! wire.

// The shipping unlisted gate formats its console line with alloc, as the
// capsule's no_std build does.
extern crate alloc;

#[path = "../../capsule_driver_i2c_pci/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_i2c_pci/src/discover/mod.rs"]
pub mod discover;
#[path = "../../capsule_driver_i2c_pci/src/driver.rs"]
pub mod driver;
pub mod gpio0;
#[path = "../../capsule_driver_i2c_pci/src/init/mod.rs"]
pub mod init;
#[path = "../../capsule_driver_i2c_pci/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_i2c_pci/src/transaction/mod.rs"]
pub mod transaction;

pub mod bench;
pub mod regs;
pub mod server;
pub mod setup;

#[cfg(test)]
mod tests;
