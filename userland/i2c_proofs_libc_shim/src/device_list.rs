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

//! MkDeviceList, answered from the records a test lists on this thread, as
//! src/syscall/microkernel/device.rs answers it: class 0 asks for every
//! record and any other class for that class's, a count of 0 asks how many
//! there are, and otherwise the first `count` are copied out and counted.
//! A test can make the call fail instead, with the errno it names.

use std::cell::RefCell;

use crate::raw;
use crate::records::DeviceRecord;

thread_local! {
    static LISTED: RefCell<Result<Vec<DeviceRecord>, i64>> = const { RefCell::new(Ok(Vec::new())) };
}

/// From now on MkDeviceList answers `records`, in their order, or fails
/// with the negative errno in `Err`.
pub fn list_devices(answer: Result<Vec<DeviceRecord>, i64>) {
    LISTED.with(|l| *l.borrow_mut() = answer);
}

pub fn mk_device_list(class: u32, buf: *mut DeviceRecord, count: u64) -> i64 {
    LISTED.with(|l| {
        let listed = l.borrow();
        let records = match &*listed {
            Ok(records) => records,
            Err(errno) => return *errno,
        };
        let shown = records.iter().filter(|r| class == 0 || r.class == class);
        if count == 0 {
            return shown.count() as i64;
        }
        let mut written = 0usize;
        for r in shown.take(usize::try_from(count).unwrap_or(usize::MAX)) {
            raw::store(buf.wrapping_add(written), *r);
            written += 1;
        }
        written as i64
    })
}
