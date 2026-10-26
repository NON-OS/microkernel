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

//! A trip lock taken back (L81b): from its writer as it leaves, from a pid
//! handed to a new process, and the two things that must never happen,
//! said once a boot if they do.

use core::sync::atomic::{AtomicBool, Ordering};

use super::trip_lock::TripLock;

static SAID_AGAIN: AtomicBool = AtomicBool::new(false);
static SAID_SWEPT: AtomicBool = AtomicBool::new(false);

impl<T> TripLock<T> {
    /// The writer leaves: it lets go, or it is swept under the rows.
    pub(super) fn left(&self, swept: bool) {
        // SAFETY: called only by that writer, or by the rows' holder over a
        // writer that never runs again, so no one else is inside.
        (self.let_go)(unsafe { &mut *self.data.get() }, swept);
    }

    /// `pid` handed to a new process: every row of its old holder goes, the
    /// waiting one too, so the new process is never taken for it.
    pub fn forget(&self, pid: u32) {
        let _quiet = super::rest_mark::quiet();
        let mut rows = self.rows.lock();
        if rows.sweep(|p| p == pid) {
            self.left(true);
        }
    }
}

/// Asked by a caller already inside: it would wait on itself for good.
pub(super) fn asked_again(me: u32, inside: bool) {
    if inside && me != 0 && !SAID_AGAIN.swap(true, Ordering::Relaxed) {
        crate::log::error!("[LOCK] pid {} waits on a trip lock it is inside", me);
    }
}

/// A guard let go with no row: its holder was swept, yet ran on.
pub(super) fn gave(me: u32, had: bool) {
    if !had && !SAID_SWEPT.swap(true, Ordering::Relaxed) {
        crate::log::error!("[LOCK] pid {} let go of a trip lock it was swept from", me);
    }
}
