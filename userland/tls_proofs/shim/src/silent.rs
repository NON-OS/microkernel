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

//! A crypto pool that takes every request and never answers, per thread.
//! The call waits out the bound it names, on the shim's clock, and comes
//! back timed out, as the kernel's does.

use std::cell::Cell;

const ETIMEDOUT: i64 = -110;

std::thread_local! {
    static SILENT: Cell<bool> = const { Cell::new(false) };
    static WAITED: Cell<(u32, Option<u64>)> = const { Cell::new((0, None)) };
}

/// Make this thread's pool silent, or let it answer again.
pub fn silence(on: bool) {
    SILENT.with(|s| s.set(on));
}

/// How many calls on this thread named a bound, and the last one named.
pub fn waits() -> (u32, Option<u64>) {
    WAITED.with(Cell::get)
}

pub fn mk_ipc_call_timeout(
    endpoint: u64,
    req: *const u8,
    req_len: usize,
    resp: *mut u8,
    cap: usize,
    timeout_ms: u64,
) -> i64 {
    WAITED.with(|w| w.set((w.get().0 + 1, Some(timeout_ms))));
    if !SILENT.with(Cell::get) {
        return crate::ipc::mk_ipc_call(endpoint, req, req_len, resp, cap);
    }
    crate::time::skip_ms(timeout_ms as i64);
    ETIMEDOUT
}
