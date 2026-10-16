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

//! Host stand-in for the kernel's service registry behind mk_service_lookup, over a set of taken names.
//
// Replaces what the MSVL syscall does behind libc's real ipc/lookup.rs, which
// the installer's real discover.rs calls. A name a test marked taken answers
// 0 and has a port and pid written; any other answers -2 (discover.rs takes
// anything but 0 as unresolved). Every name asked is recorded.

use std::cell::RefCell;

thread_local! {
    static TAKEN: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
    static ASKED: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

pub fn call(args: [u64; 6]) -> i64 {
    // SAFETY: mk_service_lookup passes a live name of the length it gives.
    let name = unsafe { std::slice::from_raw_parts(args[0] as *const u8, args[1] as usize) };
    let taken = TAKEN.with(|t| t.borrow().iter().any(|n| n == name));
    ASKED.with(|a| a.borrow_mut().push(name.to_vec()));
    if !taken {
        return -2;
    }
    // SAFETY: the two outs are live u32s of discover.rs's resolves().
    unsafe { (*(args[2] as *mut u32), *(args[3] as *mut u32)) = (0x51, 0x52) };
    0
}

/// Marks these names taken, and clears the record of lookups.
pub fn reset(taken: &[&[u8]]) {
    TAKEN.with(|t| *t.borrow_mut() = taken.iter().map(|n| n.to_vec()).collect());
    ASKED.with(|a| a.borrow_mut().clear());
}

pub fn asked() -> Vec<Vec<u8>> {
    ASKED.with(|a| std::mem::take(&mut *a.borrow_mut()))
}
