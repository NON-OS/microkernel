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

//! The PCI configuration write the controller driver makes once, after its
//! claim, kept for the test to read back. The constants are the ones in
//! userland/libc/src/broker/pci.rs.

use std::cell::RefCell;

pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
pub const MK_PCI_CMD_INTX_DISABLE: u16 = 1 << 10;

thread_local! {
    static WRITES: RefCell<Vec<(u32, u16)>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_pci_config_write(_device_id: u64, _claim_epoch: u64, offset: u32, value: u16) -> i64 {
    WRITES.with(|w| w.borrow_mut().push((offset, value)));
    0
}

/// Every (offset, value) written so far, oldest first, leaving none.
pub fn take_pci_writes() -> Vec<(u32, u16)> {
    WRITES.with(|w| std::mem::take(&mut *w.borrow_mut()))
}
