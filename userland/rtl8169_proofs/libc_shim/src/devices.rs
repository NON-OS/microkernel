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

//! The broker's device list as the driver reads it: the records a test
//! presents, answered by `mk_device_list`.

use std::cell::RefCell;

pub const BUS_KIND_PCI: u8 = 1;
pub const BAR_KIND_MMIO: u8 = 1;
pub const BAR_KIND_PIO: u8 = 2;

#[derive(Clone, Copy, Debug, Default)]
pub struct Bar {
    pub size: u64,
    pub kind: u8,
}

/// The fields of the broker's record the driver reads.
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceRecord {
    pub device_id: u64,
    pub bus_kind: u8,
    pub pci_class: u8,
    pub pci_subclass: u8,
    pub vendor: u16,
    pub device: u16,
    pub bar_count: u8,
    pub bars: [Bar; 6],
}

impl DeviceRecord {
    pub fn empty() -> Self {
        Self::default()
    }
}

thread_local! {
    static LISTED: RefCell<Vec<DeviceRecord>> = const { RefCell::new(Vec::new()) };
}

/// What the broker lists on this thread from now on.
pub fn present(records: &[DeviceRecord]) {
    LISTED.with(|l| *l.borrow_mut() = records.to_vec());
}

pub fn mk_device_list(_class: u32, buf: *mut DeviceRecord, count: u64) -> i64 {
    LISTED.with(|l| {
        let listed = l.borrow();
        let n = listed.len().min(count as usize);
        for (i, r) in listed.iter().take(n).enumerate() {
            // SAFETY: the caller hands a buffer of `count` records, as the
            // real call requires, and `i` is below it.
            unsafe { *buf.add(i) = *r };
        }
        n as i64
    })
}
