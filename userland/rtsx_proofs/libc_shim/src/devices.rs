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

//! The broker's device list: what MkDeviceList answers on this thread.

use std::cell::RefCell;

use crate::types::DeviceRecord;

thread_local! {
    static DEVICES: RefCell<Vec<DeviceRecord>> = const { RefCell::new(Vec::new()) };
}

pub fn set_devices(list: Vec<DeviceRecord>) {
    DEVICES.with(|d| *d.borrow_mut() = list);
}

pub fn mk_device_list(_class: u32, buf: *mut DeviceRecord, count: u64) -> i64 {
    DEVICES.with(|d| {
        let d = d.borrow();
        let n = d.len().min(count as usize);
        for (i, r) in d.iter().take(n).enumerate() {
            // SAFETY: the caller hands `count` writable records, as the real
            // call requires, and `i` is below it.
            unsafe { buf.add(i).write(*r) };
        }
        n as i64
    })
}
