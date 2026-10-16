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

//! The device's own side of DMA: a read the controller makes of memory it
//! was pointed at. With Bus Master off in its PCI Command register the
//! device cannot master the bus, and the read ends in a master abort.

use crate::broker::MK_PCI_CMD_BUS_MASTER;
use crate::grants::read64;
use crate::pci_command::PciCommandReg;

/// Why a read the device made got nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaFault {
    /// Bus Master is off: the read never reached memory.
    MasterAbort,
    /// No grant covers the 8 bytes, or they are not 8-byte aligned.
    Unmapped,
}

/// The device's read of the 8 bytes at bus address `bus`, `command` being
/// its PCI Command register (`pci_command_reg`).
pub fn device_read64(command: &PciCommandReg, bus: u64) -> Result<u64, DmaFault> {
    if command.get() & MK_PCI_CMD_BUS_MASTER == 0 {
        return Err(DmaFault::MasterAbort);
    }
    read64(bus).ok_or(DmaFault::Unmapped)
}
