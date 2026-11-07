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

//! Physical runs a device may still reach because an IOMMU unmap of them
//! failed. The allocator never hands them out again: a free of a frame in
//! one leaves its bit set. Empty on every machine whose units answer, so
//! the free path pays one atomic load.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;

#[path = "quarantine_rule.rs"]
mod rule;

static ANY: AtomicBool = AtomicBool::new(false);
static RUNS: Mutex<Vec<(u64, u64)>> = Mutex::new(Vec::new());

/// Keep `[start, start + len)` out of the allocator for good. Called outside
/// the allocator's lock; the run's frames may still be allocated to their
/// owner, whose free of them is then a no-op.
pub fn hold(start: u64, len: u64) {
    rule::add(&mut RUNS.lock(), start, len);
    ANY.store(true, Ordering::Release);
}

/// Whether any frame of `[start, start + len)` is held.
pub fn holds(start: u64, len: u64) -> bool {
    ANY.load(Ordering::Acquire) && rule::meets(&RUNS.lock(), start, len)
}
