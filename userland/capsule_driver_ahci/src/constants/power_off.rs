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

//! The power-off's time (server::power_off_rule): its own bound inside the
//! kernel's wait on it, and the flush's inside that, so the flush's answer
//! and a standby after it reach the kernel before it stops waiting.

use super::budget::{PRE_ISSUE_MS, RECOVER_MS};

/// The kernel's wait on the power-off (its leave leg,
/// src/hardware/ahci_capsule/client/leave.rs).
pub const KERNEL_POWER_OFF_MS: u64 = 14_500;
/// The power-off's own bound, its reply's way back inside the kernel's wait.
pub const POWER_OFF_MS: u64 = 14_000;
/// FLUSH CACHE EXT's budget in the power-off, in place of COMMAND_MS's 30 s,
/// which would hold the standby past the kernel's wait.
pub const POWER_OFF_FLUSH_MS: u64 = 5_000;
/// The least budget STANDBY IMMEDIATE is sent with.
pub const STANDBY_MIN_MS: u64 = 1_000;

/// The longest a command given `ms` holds the port: the wait before it,
/// which can outlast a short budget, or the budget; then, when it failed,
/// the recovery.
pub const fn command_worst_ms(ms: u64, failed: bool) -> u64 {
    let issued = if ms > PRE_ISSUE_MS { ms } else { PRE_ISSUE_MS };
    if failed {
        issued + RECOVER_MS
    } else {
        issued
    }
}

/// STANDBY IMMEDIATE's budget after a flush that `flushed`, its recovery
/// kept back; None when less than STANDBY_MIN_MS is left. A drive slow to
/// show ready before it (prepare, PRE_ISSUE_MS) can still hold the port
/// past this, by when the flush has answered and the cache is on the disk.
pub const fn standby_budget(flushed: bool) -> Option<u64> {
    let spent = command_worst_ms(POWER_OFF_FLUSH_MS, !flushed) + RECOVER_MS;
    match POWER_OFF_MS.checked_sub(spent) {
        Some(left) if left >= STANDBY_MIN_MS => Some(left),
        _ => None,
    }
}

const _: () = assert!(command_worst_ms(POWER_OFF_FLUSH_MS, true) < POWER_OFF_MS);
const _: () = assert!(standby_budget(true).is_some());
const _: () = assert!(POWER_OFF_MS < KERNEL_POWER_OFF_MS);
