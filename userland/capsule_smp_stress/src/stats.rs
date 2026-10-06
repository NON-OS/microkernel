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

//! The counters every worker bumps, read by the reporter.

use crate::stall::Counts;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub static STOP: AtomicBool = AtomicBool::new(false);

/// What a count is of. The first three are rounds of work, each with the
/// longest wait it saw; the last three are faults.
#[derive(Clone, Copy)]
pub enum Kind {
    Futex,
    Ipc,
    Sleep,
    Stall,
    Late,
    Error,
}

const KINDS: usize = 6;
const ROUND_KINDS: usize = 3;

static COUNT: [AtomicU64; KINDS] = [const { AtomicU64::new(0) }; KINDS];
static LONGEST_MS: [AtomicU64; ROUND_KINDS] = [const { AtomicU64::new(0) }; ROUND_KINDS];

pub fn stopping() -> bool {
    STOP.load(Ordering::Acquire)
}

/// One round of `kind`, which waited `ms` (for a sleep: past its deadline).
pub fn round(kind: Kind, ms: u64) {
    COUNT[kind as usize].fetch_add(1, Ordering::Relaxed);
    if let Some(longest) = LONGEST_MS.get(kind as usize) {
        longest.fetch_max(ms, Ordering::Relaxed);
    }
}

pub fn fault(kind: Kind) {
    COUNT[kind as usize].fetch_add(1, Ordering::Relaxed);
}

pub fn snapshot() -> Counts {
    let count = |k: Kind| COUNT[k as usize].load(Ordering::Relaxed);
    let longest = |k: Kind| LONGEST_MS[k as usize].load(Ordering::Relaxed);
    Counts {
        futex: count(Kind::Futex),
        ipc: count(Kind::Ipc),
        sleeps: count(Kind::Sleep),
        stalls: count(Kind::Stall),
        late: count(Kind::Late),
        errors: count(Kind::Error),
        max_futex_ms: longest(Kind::Futex),
        max_ipc_ms: longest(Kind::Ipc),
        max_sleep_over_ms: longest(Kind::Sleep),
    }
}
