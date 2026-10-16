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

//! The PCI Command register write, recorded with the value written. This is
//! the broker as the kernel's K6 (plan/54) makes it: a plain store of the
//! writable bits named, so a write without Bus Master turns it off. Today's
//! broker ORs them in (src/hardware/broker/pci/command.rs), and there the
//! release's own clear, read back before any frame is drained, is what
//! stops the device; the teardown tests hold that the release comes last.

use std::cell::Cell;

use crate::calls::answer;

pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
pub const MK_PCI_CMD_INTX_DISABLE: u16 = 1 << 10;

thread_local! {
    static COMMAND: Cell<u16> = const { Cell::new(0) };
}

pub fn mk_pci_config_write(_device: u64, _epoch: u64, offset: u32, value: u16) -> i64 {
    if offset != MK_PCI_CFG_COMMAND {
        return -22;
    }
    let r = answer("pci_command", value as u64);
    if r == 0 {
        COMMAND.with(|c| c.set(value));
    }
    r
}

/// The Command register as the last write that went through left it.
pub fn command() -> u16 {
    COMMAND.with(Cell::get)
}

/// Whether the device may master the bus, as the Command register stands.
pub fn bus_master() -> bool {
    command() & MK_PCI_CMD_BUS_MASTER != 0
}

/// The release clears Bus Master, as the kernel's does before it drains.
pub(crate) fn released() {
    COMMAND.with(|c| c.set(c.get() & !MK_PCI_CMD_BUS_MASTER));
}
