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

//! How many calls a guest family made and how many were unserved: the weighted
//! half of syscall coverage, printed once when the family ends so a boot's log
//! says what fraction of what programs actually asked for was answered.

use core::sync::atomic::{AtomicU64, Ordering};

static CALLS: AtomicU64 = AtomicU64::new(0);
static MISSED: AtomicU64 = AtomicU64::new(0);

pub fn call() {
    CALLS.fetch_add(1, Ordering::Relaxed);
}

pub fn missed() {
    MISSED.fetch_add(1, Ordering::Relaxed);
}

/// `[LINUX] calls served=<n> unserved=<m>`, read by tools/nonos-linux-coverage.
pub fn report() {
    let missed = MISSED.load(Ordering::Relaxed);
    let served = CALLS.load(Ordering::Relaxed).saturating_sub(missed);
    let line = alloc::format!("[LINUX] calls served={served} unserved={missed}\n");
    crate::linux::start::note(line.as_bytes());
}
