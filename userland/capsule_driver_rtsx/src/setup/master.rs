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

//! The PCI command word, and the one moment bus mastering goes on. An
//! owner that died without its teardown (a panic, a kill) was released by
//! the kernel with Bus Master clear, but its command and ADMA engines stay
//! armed at frames freed since, and would run the moment the bit is set
//! again. So the claim writes the word without it, and it goes on only
//! once both engines are stopped and the chip is seen to have taken that
//! (engine::stopped). A reader that cannot be seen to stop them never
//! masters the bus. The broker ORs the bits written into the register
//! until K6, so the claim's clear matters once the kernel stores exactly
//! the bits written; the kernel's release clears Bus Master after every
//! owner meanwhile.

use nonos_libc::{
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
    MK_PCI_CMD_MEMORY_SPACE,
};

use super::driver::Driver;
use crate::engine::stopped;
use crate::error::{Result, RtsxError};

/// Memory decode, interrupt pin quiet, no bus mastering: at claim, and at
/// teardown.
pub const QUIET: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_INTX_DISABLE;

/// Bus mastering on, once the engines are seen stopped. On a failure
/// `drv` drops, which quiesces the reader and releases it (teardown.rs).
pub fn master(drv: Driver) -> Result<Driver> {
    stopped(&drv)?;
    let (id, epoch) = (drv.claim.device_id, drv.claim.epoch);
    if mk_pci_config_write(id, epoch, MK_PCI_CFG_COMMAND, QUIET | MK_PCI_CMD_BUS_MASTER) < 0 {
        return Err(RtsxError::PciCommand);
    }
    Ok(drv)
}
