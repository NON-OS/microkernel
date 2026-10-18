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

//! Every reply the block service sends is kept, with where it went: the
//! kernel's reply endpoint or a caller's pid.

use std::cell::RefCell;

/// Where a reply went.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum To {
    Endpoint(u64),
    Pid(u32),
}

thread_local! {
    static SENT: RefCell<Vec<(To, Vec<u8>)>> = const { RefCell::new(Vec::new()) };
}

fn keep(to: To, buf: *const u8, len: usize) -> i64 {
    // SAFETY: the driver hands `len` readable bytes, as the real call requires.
    let bytes = unsafe { std::slice::from_raw_parts(buf, len) }.to_vec();
    SENT.with(|s| s.borrow_mut().push((to, bytes)));
    len as i64
}

pub fn mk_ipc_send(endpoint: u64, buf: *const u8, len: usize) -> i64 {
    keep(To::Endpoint(endpoint), buf, len)
}

pub fn mk_ipc_reply(pid: u32, buf: *const u8, len: usize) -> i64 {
    keep(To::Pid(pid), buf, len)
}

/// Every reply since the last call, in order.
pub fn take_sent() -> Vec<(To, Vec<u8>)> {
    SENT.with(|s| std::mem::take(&mut *s.borrow_mut()))
}
