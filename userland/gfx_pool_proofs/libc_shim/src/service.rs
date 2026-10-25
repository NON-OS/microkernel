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

//! The driver's pid and the compositor's service, as the kernel's registry
//! answers a lookup of it: none until a test says one is registered.

use std::cell::Cell;

/// The driver under test's process.
pub const OWN_PID: u32 = 7;

thread_local! {
    static COMPOSITOR: Cell<Option<u32>> = const { Cell::new(Some(0x27)) };
}

pub fn mk_getpid() -> u32 {
    OWN_PID
}

pub fn set_compositor(pid: Option<u32>) {
    COMPOSITOR.with(|c| c.set(pid));
}

pub fn mk_service_lookup(name: *const u8, len: usize, port: *mut u32, pid: *mut u32) -> i64 {
    // SAFETY: the caller hands `len` bytes at `name` and its own outs.
    let name = unsafe { std::slice::from_raw_parts(name, len) };
    match (name == b"compositor", COMPOSITOR.with(Cell::get)) {
        (true, Some(p)) => {
            // SAFETY: as above.
            unsafe { (*port, *pid) = (4100, p) };
            0
        }
        _ => -2,
    }
}
