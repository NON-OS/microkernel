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

//! The PCI command register as a plain store, per thread: what the broker
//! does with a command write once a capsule can clear a bit through it (the
//! kernel's K6). Today's broker ORs a write naming only writable bits into
//! the register instead. It starts as firmware may leave it, bus mastering
//! on.

use std::cell::Cell;

pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
/// I/O space decoding, a bit no capsule may change, kept as read.
pub const PCI_CMD_IO_SPACE: u16 = 1 << 0;

thread_local! {
    static COMMAND: Cell<u16> = const { Cell::new(PCI_CMD_IO_SPACE | MK_PCI_CMD_BUS_MASTER) };
}

/// The PCI command register as the driver last left it.
pub fn command() -> u16 {
    COMMAND.with(Cell::get)
}

pub fn mk_pci_config_read(_device_id: u64, _epoch: u64, offset: u32, _width: u32) -> i64 {
    if offset == MK_PCI_CFG_COMMAND {
        command() as i64
    } else {
        -22
    }
}

pub fn mk_pci_config_write(_device_id: u64, _epoch: u64, offset: u32, value: u16) -> i64 {
    if offset != MK_PCI_CFG_COMMAND {
        return -22;
    }
    COMMAND.with(|c| c.set(value));
    0
}
