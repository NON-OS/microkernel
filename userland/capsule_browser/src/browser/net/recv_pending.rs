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

//! Bytes net.sockets handed over that the reader has not taken yet.
//!
//! A read asks for as much as one reply can carry, whatever the caller's own
//! buffer, and keeps the rest here for that socket's next read. Asking for
//! only what one caller could hold moved a page 4 KB per round trip, and
//! asking for more than it could hold would have lost the remainder.

use alloc::vec::Vec;

use spin::Mutex;

static PENDING: Mutex<Vec<(u32, Vec<u8>)>> = Mutex::new(Vec::new());

/// Move up to `out.len()` held bytes for `handle` into `out`.
pub fn take(handle: u32, out: &mut [u8]) -> usize {
    let mut pending = PENDING.lock();
    let Some(at) = pending.iter().position(|(h, _)| *h == handle) else { return 0 };
    let held = &mut pending[at].1;
    let n = held.len().min(out.len());
    out[..n].copy_from_slice(&held[..n]);
    held.drain(..n);
    if held.is_empty() {
        pending.swap_remove(at);
    }
    n
}

/// Hold `bytes` for `handle`, after anything already held.
pub fn put(handle: u32, bytes: &[u8]) {
    if bytes.is_empty() {
        return;
    }
    let mut pending = PENDING.lock();
    match pending.iter_mut().find(|(h, _)| *h == handle) {
        Some((_, held)) => held.extend_from_slice(bytes),
        None => pending.push((handle, bytes.to_vec())),
    }
}

/// The socket is gone; what it held goes with it.
pub fn forget(handle: u32) {
    PENDING.lock().retain(|(h, _)| *h != handle);
}
