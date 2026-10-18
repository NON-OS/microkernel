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

//! Host stand-in for libc's syscall bridge, under the real debug.rs.
//
// Replaces libc's syscall/bridge/call_raw.rs (the SYSCALL trampoline), with
// N_MK_DEBUG copied from numbers/broker.rs over the real tag4. call_raw
// records the number and a copy of each line it is pointed at.

#[path = "../../../libc/src/syscall/numbers/tag.rs"]
mod tag;

use std::cell::RefCell;

pub const N_MK_DEBUG: i64 = tag::tag4(b"MDBG");

thread_local! {
    static LINES: RefCell<Vec<(i64, Vec<u8>)>> = const { RefCell::new(Vec::new()) };
}

pub fn call_raw(num: i64, args: [u64; 6]) -> i64 {
    // SAFETY: mk_debug passes a non-null pointer and the non-zero length of a
    // slice that lives for this call.
    let line = unsafe { core::slice::from_raw_parts(args[0] as *const u8, args[1] as usize) };
    LINES.with(|l| l.borrow_mut().push((num, line.to_vec())));
    0
}

pub fn reset() {
    LINES.with(|l| l.borrow_mut().clear());
}

/// Each line logged, in order, with the syscall number it went out on.
pub fn lines() -> Vec<(i64, Vec<u8>)> {
    LINES.with(|l| l.borrow().clone())
}
