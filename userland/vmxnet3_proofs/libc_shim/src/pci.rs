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

//! The PCI command write, recorded per thread, with a scripted answer.
//! The constants are libc's.

use std::cell::{Cell, RefCell};

pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
pub const MK_PCI_CMD_INTX_DISABLE: u16 = 1 << 10;

thread_local! {
    static WRITES: RefCell<Vec<(u64, u64, u32, u16)>> = const { RefCell::new(Vec::new()) };
    static ANSWER: Cell<i64> = const { Cell::new(0) };
}

/// What the next config writes answer.
pub fn script_pci(answer: i64) {
    ANSWER.with(|a| a.set(answer));
}

/// Every config write: device, epoch, offset, value.
pub fn pci_writes() -> Vec<(u64, u64, u32, u16)> {
    WRITES.with(|w| w.borrow().clone())
}

pub fn mk_pci_config_write(device_id: u64, claim_epoch: u64, offset: u32, value: u16) -> i64 {
    WRITES.with(|w| w.borrow_mut().push((device_id, claim_epoch, offset, value)));
    ANSWER.with(|a| a.get())
}
