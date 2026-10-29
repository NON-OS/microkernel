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

//! How long a launch took to its first window and what went into it, said
//! once when that window opens. Nothing here reaches a guest.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};
use nonos_libc::mk_uptime_ms;

#[path = "clock_line.rs"]
mod clock_line;
use clock_line::{line, Counts};

static BEGUN: AtomicU64 = AtomicU64::new(0);
static PREP_MS: AtomicU64 = AtomicU64::new(0);
static HELPERS: AtomicU64 = AtomicU64::new(0);
static READS: AtomicU64 = AtomicU64::new(0);
static READ_BYTES: AtomicU64 = AtomicU64::new(0);
static CALLS: AtomicU64 = AtomicU64::new(0);
static SAID: AtomicBool = AtomicBool::new(false);

fn now() -> u64 {
    u64::try_from(mk_uptime_ms()).unwrap_or(0)
}

pub(crate) fn begin() {
    BEGUN.store(now(), Relaxed);
}

pub(crate) fn prep(f: impl FnOnce()) {
    let from = now();
    f();
    PREP_MS.fetch_add(now().saturating_sub(from), Relaxed);
}

pub(crate) fn helper() {
    HELPERS.fetch_add(1, Relaxed);
}

pub(crate) fn read(bytes: usize) {
    READS.fetch_add(1, Relaxed);
    READ_BYTES.fetch_add(bytes as u64, Relaxed);
}

pub(crate) fn call() {
    CALLS.fetch_add(1, Relaxed);
}

pub(crate) fn window() {
    if SAID.swap(true, Relaxed) {
        return;
    }
    let c = Counts {
        ms: now().saturating_sub(BEGUN.load(Relaxed)),
        prep_ms: PREP_MS.load(Relaxed),
        helpers: HELPERS.load(Relaxed),
        reads: READS.load(Relaxed),
        read_bytes: READ_BYTES.load(Relaxed),
        calls: CALLS.load(Relaxed),
    };
    super::routine(line(&c).as_bytes());
}
