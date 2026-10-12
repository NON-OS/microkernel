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

//! Hardware errors the ACPI capsule decoded from GHES sources and BERT
//! (ACPI 6.5, 18.3.2.7), counted by severity for the posture line. The
//! capsule logs each record itself; the kernel only counts.

use core::sync::atomic::{AtomicU64, Ordering};

/// Generic Error Severity (18.3.2.7.1): 0 recoverable, 1 fatal,
/// 2 corrected, 3 none.
static BY_SEVERITY: [AtomicU64; 4] =
    [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];

/// False for a severity the specification does not define.
pub fn count(severity: u64) -> bool {
    let Some(c) = usize::try_from(severity).ok().and_then(|s| BY_SEVERITY.get(s)) else {
        return false;
    };
    c.fetch_add(1, Ordering::Relaxed);
    super::report::posture();
    true
}

/// Recoverable, fatal, corrected.
pub(super) fn counts() -> (u64, u64, u64) {
    let n = |i: usize| BY_SEVERITY[i].load(Ordering::Relaxed);
    (n(0), n(1), n(2))
}
