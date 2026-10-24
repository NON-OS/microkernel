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

//! How many config accesses went through the validated and scan paths.

use core::sync::atomic::{AtomicU64, Ordering};

static READS: AtomicU64 = AtomicU64::new(0);
static WRITES: AtomicU64 = AtomicU64::new(0);

pub(super) fn read() {
    READS.fetch_add(1, Ordering::Relaxed);
}

pub(super) fn wrote() {
    WRITES.fetch_add(1, Ordering::Relaxed);
}

pub fn get_config_stats() -> (u64, u64) {
    (READS.load(Ordering::Relaxed), WRITES.load(Ordering::Relaxed))
}

pub fn reset_config_stats() {
    READS.store(0, Ordering::Relaxed);
    WRITES.store(0, Ordering::Relaxed);
}
