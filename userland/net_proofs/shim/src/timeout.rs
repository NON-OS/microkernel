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

//! A bounded call, and the timeout the capsule's last one asked for, so a
//! proof can see what bound a call was given.

use std::sync::atomic::{AtomicU64, Ordering};

static LAST_TIMEOUT_MS: AtomicU64 = AtomicU64::new(0);

/// `mk_ipc_call`, its timeout kept as the last asked for.
pub fn mk_ipc_call_timeout(
    endpoint: u64,
    req: *const u8,
    req_len: usize,
    resp: *mut u8,
    cap: usize,
    timeout_ms: u64,
) -> i64 {
    LAST_TIMEOUT_MS.store(timeout_ms, Ordering::Relaxed);
    crate::ipc::mk_ipc_call(endpoint, req, req_len, resp, cap)
}

/// The timeout the capsule's last `mk_ipc_call_timeout` asked for.
pub fn last_timeout_ms() -> u64 {
    LAST_TIMEOUT_MS.load(Ordering::Relaxed)
}
