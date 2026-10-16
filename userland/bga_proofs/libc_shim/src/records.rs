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

//! The device list discovery reads, as the test offers it.

use std::cell::RefCell;

pub const BUS_KIND_PCI: u8 = 1;
pub const BAR_KIND_MMIO: u8 = 1;

#[derive(Clone, Copy, Default)]
pub struct Bar {
    pub size: u64,
    pub kind: u8,
}

#[derive(Clone, Copy, Default)]
pub struct DeviceRecord {
    pub device_id: u64,
    pub bus_kind: u8,
    pub pci_class: u8,
    pub vendor: u16,
    pub device: u16,
    pub bar_count: u8,
    pub bars: [Bar; 6],
}

impl DeviceRecord {
    pub const fn empty() -> Self {
        let b = Bar { size: 0, kind: 0 };
        Self {
            device_id: 0,
            bus_kind: 0,
            pci_class: 0,
            vendor: 0,
            device: 0,
            bar_count: 0,
            bars: [b; 6],
        }
    }
}

thread_local! {
    static OFFERED: RefCell<Vec<DeviceRecord>> = const { RefCell::new(Vec::new()) };
}

/// The records the device list answers with on this thread.
pub fn offer(records: Vec<DeviceRecord>) {
    OFFERED.with(|o| *o.borrow_mut() = records);
}

pub fn mk_device_list(_class: u32, buf: *mut DeviceRecord, count: u64) -> i64 {
    let offered = OFFERED.with(|o| o.borrow().clone());
    let n = offered.len().min(count as usize);
    // SAFETY: the caller hands a buffer of `count` records, as the real call
    // requires, and at most that many are written.
    unsafe { std::ptr::copy_nonoverlapping(offered.as_ptr(), buf, n) };
    n as i64
}
