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

//! A clock the proof owns. A yield moves it one millisecond, so a capsule loop
//! that waits for a deadline reaches it.

use std::sync::atomic::{AtomicI64, Ordering};

static NOW_MS: AtomicI64 = AtomicI64::new(1_000_000);

pub fn set_time(ms: i64) {
    NOW_MS.store(ms, Ordering::SeqCst);
}

pub fn advance(ms: i64) {
    NOW_MS.fetch_add(ms, Ordering::SeqCst);
}

pub fn mk_time_millis() -> i64 {
    NOW_MS.load(Ordering::SeqCst)
}

pub fn mk_uptime_ms() -> i64 {
    NOW_MS.load(Ordering::SeqCst)
}

pub fn mk_yield() -> i64 {
    advance(1);
    0
}

/// The clock is the proof's; a capsule asking to set it is refused.
pub fn mk_time_adjust(_correct_ms: u64) -> i64 {
    -1
}
