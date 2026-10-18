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

//! The PCI command register as the driver writes it: each write kept, and
//! shown to the test at the moment it is made, so the test can read the
//! device then (bus mastering may go on only once it is seen reset).

use std::cell::RefCell;

pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
pub const MK_PCI_CMD_INTX_DISABLE: u16 = 1 << 10;

thread_local! {
    static WRITES: RefCell<Vec<(u32, u16)>> = const { RefCell::new(Vec::new()) };
    static WATCH: RefCell<Option<Box<dyn Fn(u16)>>> = const { RefCell::new(None) };
}

pub fn mk_pci_config_write(_device_id: u64, _claim_epoch: u64, offset: u32, value: u16) -> i64 {
    WRITES.with(|w| w.borrow_mut().push((offset, value)));
    WATCH.with(|w| {
        if let Some(watch) = w.borrow().as_ref() {
            watch(value);
        }
    });
    0
}

/// Every configuration write this thread's driver made, oldest first.
pub fn pci_writes() -> Vec<(u32, u16)> {
    WRITES.with(|w| w.borrow().clone())
}

/// Run `watch` with each value written, before the write returns.
pub fn on_pci_write(watch: impl Fn(u16) + 'static) {
    WATCH.with(|w| *w.borrow_mut() = Some(Box::new(watch)));
}
