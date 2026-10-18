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

//! When the wait for the boot disk began: the first refusal of a pass that
//! may stand between this disk and the loader's, stamped once and kept.
//! Pure, so kernel_proofs holds it.

use core::sync::atomic::{AtomicU64, Ordering};

/// Stamp `now` unless a first refusal is already stamped; the stamp is
/// `now + 1`, so 0 stays "not yet".
pub(super) fn stamp(since: &AtomicU64, now: u64) -> u64 {
    let mark = now.saturating_add(1);
    match since.compare_exchange(0, mark, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => mark,
        Err(first) => first,
    }
}

/// Whether `limit_ms` have passed since the stamp; false before any.
pub(super) fn over(since: &AtomicU64, now: u64, limit_ms: u64) -> bool {
    match since.load(Ordering::Acquire) {
        0 => false,
        first => now.saturating_add(1).saturating_sub(first) >= limit_ms,
    }
}
