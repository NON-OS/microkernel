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

//! The device list, as a test sets it.

use std::cell::RefCell;

pub const BUS_KIND_PCI: u8 = 1;
pub const BUS_KIND_VIRT: u8 = 3;

#[derive(Clone, Copy, Default)]
pub struct DeviceRecord {
    pub device_id: u64,
    pub bus_kind: u8,
    pub vendor: u16,
    pub device: u16,
}

impl DeviceRecord {
    pub fn empty() -> Self {
        Self::default()
    }
}

thread_local! {
    static DEVICES: RefCell<Vec<DeviceRecord>> = const { RefCell::new(Vec::new()) };
}

pub fn set_devices(list: Vec<DeviceRecord>) {
    DEVICES.with(|d| *d.borrow_mut() = list);
}

pub fn mk_device_list(_filter: u64, out: *mut DeviceRecord, max: u64) -> i64 {
    DEVICES.with(|d| {
        let list = d.borrow();
        let n = list.len().min(max as usize);
        // SAFETY: the caller hands room for `max` records.
        unsafe { std::ptr::copy_nonoverlapping(list.as_ptr(), out, n) };
        n as i64
    })
}
