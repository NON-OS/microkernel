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

//! The PCI command register. The driver changes it by the masked write
//! (`pci_update.rs`); the plain write here stores a value whole, which a
//! test uses to stand in for the kernel's release. A test can have the
//! broker refuse both, as it does a claim it no longer honours.

use std::cell::Cell;

use crate::broker::record;

pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;

thread_local! {
    pub(crate) static COMMAND: Cell<u16> =
        const { Cell::new(MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_MEMORY_SPACE) };
    pub(crate) static REFUSED: Cell<bool> = const { Cell::new(false) };
}

/// The PCI command register as the driver last left it.
pub fn command() -> u16 {
    COMMAND.with(Cell::get)
}

/// Refuse every read and write of the command register from now on.
pub fn refuse_command() {
    REFUSED.with(|r| r.set(true));
}

pub fn mk_pci_config_read(_device_id: u64, _epoch: u64, offset: u32, _width: u32) -> i64 {
    record("pci_config_read", offset as u64);
    if offset != MK_PCI_CFG_COMMAND {
        return -22;
    }
    if REFUSED.with(Cell::get) {
        return -1;
    }
    command() as i64
}

pub fn mk_pci_config_write(_device_id: u64, _epoch: u64, offset: u32, value: u16) -> i64 {
    record("pci_config_write", value as u64);
    if offset != MK_PCI_CFG_COMMAND {
        return -22;
    }
    if REFUSED.with(Cell::get) {
        return -1;
    }
    COMMAND.with(|c| c.set(value));
    0
}
