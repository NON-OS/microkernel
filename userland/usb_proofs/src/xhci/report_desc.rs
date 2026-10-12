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

//! A device's HID class answers, set by a test, per thread: the report
//! descriptor it gives to GET_DESCRIPTOR(Report), no more than was asked
//! for, and whether it takes SET_PROTOCOL(boot). The reads asked are
//! counted, so a test can tell a descriptor refused unread from one read
//! and refused.

use std::cell::{Cell, RefCell};

thread_local! {
    static DESC: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static READS: Cell<u32> = const { Cell::new(0) };
    static STALLS_BOOT: Cell<bool> = const { Cell::new(false) };
}

/// The device answers with `desc` from now on, and takes SET_PROTOCOL; no
/// read asked yet.
pub fn serve(desc: &[u8]) {
    DESC.with(|d| *d.borrow_mut() = desc.to_vec());
    READS.with(|r| r.set(0));
    STALLS_BOOT.with(|s| s.set(false));
}

/// The device STALLs SET_PROTOCOL(boot) from now on.
pub fn stall_boot_protocol() {
    STALLS_BOOT.with(|s| s.set(true));
}

pub(super) fn stalls_boot() -> bool {
    STALLS_BOOT.with(Cell::get)
}

/// The reads asked since the last `serve`.
pub fn reads() -> u32 {
    READS.with(Cell::get)
}

/// The descriptor into `data`, cut at `asked` bytes; the bytes given.
pub(super) fn give(data: &mut [u8], asked: u16) -> usize {
    READS.with(|r| r.set(r.get() + 1));
    DESC.with(|d| {
        let d = d.borrow();
        let n = d.len().min(usize::from(asked)).min(data.len());
        data[..n].copy_from_slice(&d[..n]);
        n
    })
}
