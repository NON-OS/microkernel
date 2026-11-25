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

//! When the display's inbox is worth reading. Every pass of the serve loop
//! read it, at least one IPC receive a pass, so a program's every call paid
//! for input that had not come: a quarter of a Qt program's start. The
//! kernel's wait says when a message came (it returns 0) and when nothing
//! did (a timeout); the inbox is read then, and every EVERY_MS besides, so
//! a program busy with calls still takes its input in time, and a batch
//! that left more behind is read again within it.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};

pub const EVERY_MS: u64 = 8;

static DUE: AtomicBool = AtomicBool::new(true);
static LAST: AtomicU64 = AtomicU64::new(0);

/// The wait's answer `rc`, passed on; a message or a timeout makes the
/// inbox due.
pub fn waited(rc: i64) -> i64 {
    if rc <= 0 {
        DUE.store(true, Relaxed);
    }
    rc
}

/// Whether to read the inbox at `now` ms, and it is then taken as read.
pub fn take(now: u64) -> bool {
    let read = due(DUE.load(Relaxed), now, LAST.load(Relaxed));
    if read {
        DUE.store(false, Relaxed);
        LAST.store(now, Relaxed);
    }
    read
}

/// Read when told something came, or EVERY_MS after the last read.
pub fn due(told: bool, now: u64, last: u64) -> bool {
    told || now.saturating_sub(last) >= EVERY_MS
}
