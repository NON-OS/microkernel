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

//! A lock its holder may keep across a capsule round trip (L81b), waited
//! for by resting as a reply is, never by spinning, so on one CPU with
//! interrupts masked the holder still runs to let go. A holder that never
//! runs again (L77e's test) is swept; `let_go` puts right after a writer.

use core::cell::UnsafeCell;
use spin::Mutex;

use super::trip_rows::Rows;

pub struct TripLock<T> {
    pub(super) rows: Mutex<Rows>,
    pub(super) data: UnsafeCell<T>,
    /// Run on the data as its writer leaves: let go (false) or swept (true).
    pub(super) let_go: fn(&mut T, bool),
}

// SAFETY: the data is reached only through a guard the rows grant, one
// writer or any readers, as a reader-writer lock grants it.
unsafe impl<T: Send + Sync> Sync for TripLock<T> {}

impl<T> TripLock<T> {
    pub const fn new(data: T, let_go: fn(&mut T, bool)) -> Self {
        TripLock { rows: Mutex::new(Rows::new()), data: UnsafeCell::new(data), let_go }
    }

    /// In, as the writer when `whole`: the pid the rows now name.
    pub(super) fn wait(&self, whole: bool) -> u32 {
        let (me, mut round) = (crate::process::current_pid().unwrap_or(0), 0u32);
        loop {
            /* Read before the look, so a give after it ends the sleep. */
            let mark = super::reply_wait::wake_mark();
            if self.look(me, whole) {
                return me;
            }
            super::reply_rest::rest_since(round, mark);
            round = round.saturating_add(1);
        }
    }

    /// One look under the rows: in, or marked as waiting.
    fn look(&self, me: u32, whole: bool) -> bool {
        let _quiet = super::rest_mark::quiet();
        let (mut rows, now) = (self.rows.lock(), crate::time::timestamp_millis());
        let gone = super::transport_holder::never_runs_again;
        if !rows.may_take(me, whole, now) && rows.sweep(gone) {
            self.left(true);
        }
        let take = rows.may_take(me, whole, now);
        if take {
            rows.take(me, whole);
        } else {
            super::trip_back::asked_again(me, rows.has(me));
            rows.wait(me, whole, now);
        }
        take
    }
}
