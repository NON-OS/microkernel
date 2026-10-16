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

//! The broker's device list: whatever records the test presents.

use std::cell::RefCell;

use crate::types::device::DeviceRecord;

thread_local! {
    static RECORDS: RefCell<Vec<DeviceRecord>> = const { RefCell::new(Vec::new()) };
}

/// What the next lists answer, in this order.
pub fn present(records: &[DeviceRecord]) {
    RECORDS.with(|r| *r.borrow_mut() = records.to_vec());
}

/// Up to `count` records into `buf`, as MkDeviceList fills it; the count.
pub fn mk_device_list(_class: u32, buf: *mut DeviceRecord, count: u64) -> i64 {
    RECORDS.with(|r| {
        let r = r.borrow();
        let n = r.len().min(count as usize);
        // SAFETY: the driver hands a buffer of `count` records it owns.
        let out = unsafe { std::slice::from_raw_parts_mut(buf, count as usize) };
        out[..n].copy_from_slice(&r[..n]);
        n as i64
    })
}
