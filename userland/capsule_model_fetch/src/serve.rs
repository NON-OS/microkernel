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

/*
 * Answering the store's one question on this program's endpoint: where
 * the download stands (`status_wire`). It is asked between batches, while a
 * broken connection waits to be tried again, and in every wait on the
 * network: a stream opening through the Anyone network, a handshake or a
 * mirror's headers, a read with nothing come yet. The store waits 150 ms for
 * an answer, so the endpoint is looked at every 50: at four times a second,
 * and only between batches, most questions were asked while nobody was
 * listening, and the card froze through the slowest part of an install.
 * Looking costs one empty receive. Anything but the question is read and
 * dropped; nothing here acts on what arrives.
 */

use core::sync::atomic::{AtomicI64, AtomicU64, AtomicU8, Ordering::Relaxed};

use nonos_libc::{mk_idle_ms, mk_ipc_recv_from, mk_ipc_reply, mk_uptime_ms};

use crate::status_wire::{Status, ASK, BROKEN, FETCHING, STARTING};

/* The service port the kernel registers for tool.model-fetch. */
const PORT: u64 = 4960;
const EVERY_MS: i64 = 50;
/* Questions answered in one go, so a caller in a loop cannot hold a download. */
const AT_ONCE: usize = 4;

static STAGE: AtomicU8 = AtomicU8::new(0);
static ROUTE: AtomicU8 = AtomicU8::new(0);
static TOTAL: AtomicU64 = AtomicU64::new(0);
static DONE: AtomicU64 = AtomicU64::new(0);
static RATE: AtomicU64 = AtomicU64::new(0);
static TRY: AtomicU8 = AtomicU8::new(0);
static TRIES: AtomicU8 = AtomicU8::new(0);
static NEXT_MS: AtomicI64 = AtomicI64::new(0);

/* What the next answer says. */
pub fn set(s: Status) {
    STAGE.store(s.stage, Relaxed);
    ROUTE.store(s.route, Relaxed);
    TOTAL.store(s.total, Relaxed);
    DONE.store(s.done, Relaxed);
    RATE.store(s.rate, Relaxed);
    TRY.store(s.try_n, Relaxed);
    TRIES.store(s.tries, Relaxed);
}

/* Only the stage changes: the kernel checking the last bytes. */
pub fn stage(stage: u8) {
    STAGE.store(stage, Relaxed);
}

/* A connection failed or dropped; try `n` of `of` comes after a wait. */
pub fn waiting(n: u32, of: u32) {
    TRY.store(n.min(255) as u8, Relaxed);
    TRIES.store(of.min(255) as u8, Relaxed);
    STAGE.store(BROKEN, Relaxed);
}

/* The wait is over and the next try is under way. */
pub fn going_on() {
    TRY.store(0, Relaxed);
    TRIES.store(0, Relaxed);
    STAGE.store(FETCHING, Relaxed);
}

fn now() -> Status {
    Status {
        stage: STAGE.load(Relaxed),
        route: ROUTE.load(Relaxed),
        total: TOTAL.load(Relaxed),
        done: DONE.load(Relaxed),
        rate: RATE.load(Relaxed),
        try_n: TRY.load(Relaxed),
        tries: TRIES.load(Relaxed),
    }
}

/* Wait `ms`, answering the store all the while. */
pub fn idle(ms: u64) {
    let until = mk_uptime_ms().saturating_add(ms as i64);
    loop {
        answer();
        let left = until.saturating_sub(mk_uptime_ms());
        if left <= 0 {
            return;
        }
        mk_idle_ms((left as u64).min(EVERY_MS as u64));
    }
}

/* Answer what has been asked since the last time, if it is time to. */
pub fn answer() {
    let t = mk_uptime_ms();
    if t < NEXT_MS.load(Relaxed) {
        return;
    }
    NEXT_MS.store(t.saturating_add(EVERY_MS), Relaxed);
    let mut buf = [0u8; 16];
    for _ in 0..AT_ONCE {
        let mut from = 0u32;
        let n = mk_ipc_recv_from(PORT, buf.as_mut_ptr(), buf.len(), 1, &mut from);
        if n <= 0 || from == 0 {
            return;
        }
        let mut s = now();
        if buf.get(..n as usize) == Some(&ASK[..]) {
            // Nothing set yet: the fetcher is finding its route and sizes.
            // Answered as starting, never left for the store to time out on.
            if s.stage == 0 {
                s.stage = STARTING;
            }
            let out = s.encode();
            let _ = mk_ipc_reply(from, out.as_ptr(), out.len());
        }
    }
}
