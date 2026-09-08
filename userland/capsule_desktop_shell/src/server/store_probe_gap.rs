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

//! How long to wait before asking vfs_pool again.
//!
//! The probe used to run on every tick, and vfs_pool spends its first seconds
//! staging packages off the block device where it cannot answer anyone. Every
//! tick in that window spent a whole IPC timeout inside the shell's own loop:
//! around forty dead calls per boot, each one a stall in the thing drawing the
//! desktop. A probe that asks harder while a service is busy is a probe that
//! keeps it busy.

use core::sync::atomic::{AtomicU64, Ordering};

use nonos_libc::mk_uptime_ms;

/// First gap after a miss, and the ceiling it doubles towards. Staging takes a
/// few seconds, so the ceiling still answers promptly once vfs_pool is free
/// without polling it while it is not.
const FIRST_RETRY_MS: u64 = 250;
const MAX_RETRY_MS: u64 = 4000;

static NEXT_TRY_MS: AtomicU64 = AtomicU64::new(0);
static GAP_MS: AtomicU64 = AtomicU64::new(FIRST_RETRY_MS);

/// Whether enough time has passed to spend another call.
pub fn due() -> bool {
    now() >= NEXT_TRY_MS.load(Ordering::Relaxed)
}

/// Record a miss and widen the gap.
pub fn missed() {
    let gap = GAP_MS.load(Ordering::Relaxed);
    NEXT_TRY_MS.store(now().saturating_add(gap), Ordering::Relaxed);
    GAP_MS.store((gap * 2).min(MAX_RETRY_MS), Ordering::Relaxed);
}

fn now() -> u64 {
    mk_uptime_ms().max(0) as u64
}
