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

//! The PCI Command register the two writes in `pci_write.rs` act on. The
//! device release clears Bus Master before it frees anything, as
//! `MkDeviceRelease` does.

use std::cell::Cell;

use crate::steps::{note, Step};

pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
pub(crate) const WRITABLE: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER | (1 << 10);

thread_local! {
    static COMMAND: Cell<u16> = const { Cell::new(0) };
    static REFUSE: Cell<bool> = const { Cell::new(false) };
}

/// The Command register as the claim hands it over.
pub fn set_command(value: u16) {
    COMMAND.with(|c| c.set(value));
    REFUSE.with(|r| r.set(false));
}

pub fn command() -> u16 {
    COMMAND.with(|c| c.get())
}

/// From now on the broker refuses any write that turns Bus Master on.
pub fn refuse_bus_master() {
    REFUSE.with(|r| r.set(true));
}

pub(crate) fn bus_master_on() -> bool {
    command() & MK_PCI_CMD_BUS_MASTER != 0
}

pub(crate) fn stop_bus_master() {
    COMMAND.with(|c| c.set(c.get() & !MK_PCI_CMD_BUS_MASTER));
}

/// The register with the bits of `mask` as `value` has them, unless the
/// value turns Bus Master on while the broker refuses that.
pub(crate) fn store(value: u16, mask: u16) -> i64 {
    note(Step::Command(value));
    if value & MK_PCI_CMD_BUS_MASTER != 0 && REFUSE.with(|r| r.get()) {
        return -1;
    }
    COMMAND.with(|c| c.set((c.get() & !mask) | value));
    0
}
