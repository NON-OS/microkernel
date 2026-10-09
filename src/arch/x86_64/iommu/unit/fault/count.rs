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

//! Every fault record drained since boot, for the summary line and for any
//! caller that wants to know whether the machine has seen a denial at all.

use core::sync::atomic::{AtomicU64, Ordering};

static TOTAL: AtomicU64 = AtomicU64::new(0);

pub(super) fn count(drained: usize) -> u64 {
    TOTAL.fetch_add(drained as u64, Ordering::Relaxed) + drained as u64
}

/// Fault records drained since boot, across every unit.
pub fn fault_total() -> u64 {
    TOTAL.load(Ordering::Relaxed)
}
