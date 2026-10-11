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

//! Claim and the port grant, each recorded with every other broker call.

use std::cell::RefCell;

#[repr(C)]
pub struct PioGrantOut {
    pub port_base: u16,
    pub port_count: u16,
    pub _pad: u32,
    pub grant_id: u64,
}

thread_local! {
    static CALLS: RefCell<Vec<(&'static str, u64)>> = const { RefCell::new(Vec::new()) };
}

pub(crate) fn note(call: &'static str, id: u64) -> i64 {
    CALLS.with(|c| c.borrow_mut().push((call, id)));
    0
}

/// Every broker call made on this thread, as (call, id), in order.
pub fn calls() -> Vec<(&'static str, u64)> {
    CALLS.with(|c| c.borrow().clone())
}

pub fn mk_device_claim(device_id: u64) -> i64 {
    note("claim", device_id);
    7
}
pub fn mk_device_release(device_id: u64) -> i64 {
    note("device_release", device_id)
}
pub fn mk_pio_grant(_d: u64, _e: u64, bar: u8, _f: u32, out: *mut PioGrantOut) -> i64 {
    // SAFETY: the caller hands a writable PioGrantOut, as the real call requires.
    unsafe {
        *out = PioGrantOut { port_base: 0x1070, port_count: 16, _pad: 0, grant_id: 90 + bar as u64 }
    };
    note("pio_grant", bar as u64)
}
pub fn mk_pio_release(grant: u64) -> i64 {
    note("pio_release", grant)
}
