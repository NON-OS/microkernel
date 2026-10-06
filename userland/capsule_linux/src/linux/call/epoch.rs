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

//! Where a family's clocks start.
//!
//! Uptime is the machine's: it dates the boot, which is the same for every
//! guest on it and differs between machines, so read raw it both fingerprints
//! the machine and lets two guests agree on a moment. A guest's monotonic
//! clocks count from when its family started instead.

use core::sync::atomic::{AtomicU64, Ordering};

use nonos_libc::mk_uptime_ms;

static START: AtomicU64 = AtomicU64::new(0);

fn uptime() -> u64 {
    u64::try_from(mk_uptime_ms()).unwrap_or(0)
}

/// Called once, before the first guest runs.
pub fn mark_start() {
    START.store(uptime(), Ordering::Relaxed);
}

/// Milliseconds since the family started.
pub fn family_ms() -> u64 {
    uptime().saturating_sub(START.load(Ordering::Relaxed))
}
