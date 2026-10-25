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

//! The device broker, for one device per test thread: a claim, a PCI
//! Command register written as a plain store, and the rest of config space
//! as the test offers it (`pci_config`). Every call that gives something
//! back or changes the device is recorded in order (`log`).

use crate::log::{record, Call};
use crate::pci_command::write;
use crate::{pci_config, pci_config_write};

/// Offset of the PCI Command register, written as a plain store.
pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
pub const MK_PCI_CMD_INTX_DISABLE: u16 = 1 << 10;

pub fn mk_device_claim(_device_id: u64) -> i64 {
    1
}

pub fn mk_device_release(device_id: u64) -> i64 {
    record(Call::DeviceRelease(device_id));
    0
}

pub fn mk_pci_config_write(_device_id: u64, _epoch: u64, offset: u32, value: u16) -> i64 {
    if offset != MK_PCI_CFG_COMMAND {
        return pci_config_write::write(offset, value);
    }
    write(value);
    record(Call::PciCommand(value));
    0
}

pub fn mk_pci_config_read(_device_id: u64, _epoch: u64, offset: u32, width: u32) -> i64 {
    pci_config::read(offset, width)
}
