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

//! The one libc call the hub capsule's wire makes, served on the host: the
//! reply is the bytes the test set, as many as fit, and the call returns
//! how many it delivered, as the kernel's does.

use std::cell::RefCell;

thread_local! {
    static REPLY: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// The bytes the next calls deliver.
pub fn set_reply(bytes: Vec<u8>) {
    REPLY.with(|r| *r.borrow_mut() = bytes);
}

pub fn mk_ipc_call(_port: u64, _req: *const u8, _len: usize, resp: *mut u8, cap: usize) -> i64 {
    REPLY.with(|r| {
        let r = r.borrow();
        let n = r.len().min(cap);
        // SAFETY: the caller hands `cap` writable bytes, as the real call requires.
        unsafe { core::ptr::copy_nonoverlapping(r.as_ptr(), resp, n) };
        n as i64
    })
}
