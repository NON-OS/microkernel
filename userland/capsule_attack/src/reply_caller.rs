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

//! The caller reply-twice answers (reply_twice.rs): a worker thread whose
//! one call to test.attack's own port is still waiting when the answer
//! comes, so the answer has a call to reach, and what that call got back.

use core::sync::atomic::{AtomicI64, AtomicU8, Ordering};

use nonos_libc::thread::WorkerSeat;
use nonos_libc::{mk_ipc_call_timeout, mk_uptime_ms, mk_yield};

use crate::reply_take::OWN_PORT;

/// How long the call waits for its answer, and its return until it has one.
const WAIT_MS: u64 = 5_000;
const PENDING: i64 = i64::MIN;

static SEAT: WorkerSeat = WorkerSeat::new();
static RC: AtomicI64 = AtomicI64::new(PENDING);
static FIRST: AtomicU8 = AtomicU8::new(0);

fn call(body: usize) {
    let (req, mut back) = ([body as u8], [0u8; 8]);
    let rc = mk_ipc_call_timeout(OWN_PORT, req.as_ptr(), 1, back.as_mut_ptr(), back.len(), WAIT_MS);
    FIRST.store(back[0], Ordering::Relaxed);
    RC.store(rc, Ordering::Release);
}

/// Start the call carrying `body` on the worker: the thread's id, which is
/// the sender its request names, or the spawn's errno.
pub fn start(body: u8) -> Result<u32, i64> {
    SEAT.spawn(call, usize::from(body))
}

/// What the call returned and the first byte it got back, once it has
/// returned; None if it has not within its own wait and a second more.
pub fn returned() -> Option<(i64, u8)> {
    let since = mk_uptime_ms();
    loop {
        match RC.load(Ordering::Acquire) {
            PENDING if mk_uptime_ms().saturating_sub(since) > WAIT_MS as i64 + 1_000 => {
                return None
            }
            PENDING => {
                let _ = mk_yield();
            }
            rc => return Some((rc, FIRST.load(Ordering::Relaxed))),
        }
    }
}
