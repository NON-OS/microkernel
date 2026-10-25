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

//! The service registry, per thread: every name and port the driver
//! registers, and the process the kernel registered each other name to.

use std::cell::RefCell;

/// What the kernel answers a lookup of a name nobody holds.
const ERRNO_NOENT: i64 = -2;

thread_local! {
    static SERVICES: RefCell<Vec<(Vec<u8>, u32)>> = const { RefCell::new(Vec::new()) };
    static HOLDERS: RefCell<Vec<(Vec<u8>, u32)>> = const { RefCell::new(Vec::new()) };
}

/// `name` registered to `pid` from now on, as the kernel registers a
/// capsule's endpoint at its spawn, or to nobody once it is gone.
pub fn hold(name: &[u8], pid: Option<u32>) {
    HOLDERS.with(|h| {
        let mut h = h.borrow_mut();
        h.retain(|(held, _)| held != name);
        h.extend(pid.map(|pid| (name.to_vec(), pid)));
    });
}

/// The pid `name` is registered to. The port is not modelled: it reads 0.
pub fn mk_service_lookup(name: *const u8, len: usize, port: *mut u32, pid: *mut u32) -> i64 {
    // SAFETY: the caller hands `len` bytes at `name`, as the real call does.
    let name = unsafe { std::slice::from_raw_parts(name, len) };
    let held = HOLDERS.with(|h| h.borrow().iter().find(|(n, _)| n == name).map(|e| e.1));
    let Some(held) = held else { return ERRNO_NOENT };
    // SAFETY: each out pointer is null or a u32 the caller holds, as the
    // real call requires.
    unsafe {
        if !port.is_null() {
            *port = 0;
        }
        if !pid.is_null() {
            *pid = held;
        }
    }
    0
}

pub fn mk_service_register(name: *const u8, name_len: usize, port: u32) -> i64 {
    // SAFETY: the caller hands `name_len` bytes at `name`, as the real call
    // requires.
    let name = unsafe { std::slice::from_raw_parts(name, name_len) }.to_vec();
    SERVICES.with(|s| s.borrow_mut().push((name, port)));
    0
}

/// Every name and port registered on this thread, in order.
pub fn services() -> Vec<(Vec<u8>, u32)> {
    SERVICES.with(|s| s.borrow().clone())
}

/// The driver's own pid, the pool's: a share naming it answers the owner's handle.
pub fn mk_getpid() -> u32 {
    3
}
