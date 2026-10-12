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

//! Per-call correlation tokens: monotonic and never zero, so one can never
//! collide with the correlation an attacker's `mk_ipc_send` produces (always
//! 0) nor with another call in flight. The reply must carry this exact value.
//!
//! A call's token is registered with the pending reply keyed by the caller's
//! inbox. Both reply paths read it back from that same entry: the redirect
//! path (a service replying via `mk_ipc_send` to its fixed reply endpoint)
//! and the direct `mk_ipc_reply` path both stamp the reply with it. The
//! genuine reply carries this token; a forged injection can only carry 0.

use core::sync::atomic::{AtomicU64, Ordering};

static CALL_TOKEN: AtomicU64 = AtomicU64::new(1);

pub(super) fn next_call_token() -> u64 {
    let t = CALL_TOKEN.fetch_add(1, Ordering::Relaxed);
    if t == 0 {
        CALL_TOKEN.fetch_add(1, Ordering::Relaxed)
    } else {
        t
    }
}
