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

//! The number of the next read on each open socket.
//!
//! net.sockets takes bytes out of the socket before its reply reaches us, and
//! a reply that arrives after our 200 ms wait is dropped by the kernel. Each
//! read therefore carries a number: a read that timed out is asked again with
//! the same number and net.sockets hands back the same bytes, and only a read
//! that was answered moves to the next number.

use alloc::vec::Vec;

use spin::Mutex;

static NEXT: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());

/// The number to send with the next read on `handle`.
pub fn current(handle: u32) -> u32 {
    NEXT.lock().iter().find(|(h, _)| *h == handle).map_or(1, |(_, s)| *s)
}

/// A read on `handle` was answered with bytes: the next read is a new one.
pub fn answered(handle: u32) {
    let mut next = NEXT.lock();
    match next.iter_mut().find(|(h, _)| *h == handle) {
        Some((_, s)) => *s = s.wrapping_add(1).max(1),
        None => next.push((handle, 2)),
    }
}

/// The socket is gone; a new one given the same handle starts again at 1.
pub fn forget(handle: u32) {
    NEXT.lock().retain(|(h, _)| *h != handle);
}
