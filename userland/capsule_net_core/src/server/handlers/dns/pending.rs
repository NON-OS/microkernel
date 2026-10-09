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

//! The DNS lookups started and not yet answered. A lookup used to be pumped to
//! its end inside dispatch, up to three seconds in which net.core answered no
//! one else; now the request starts the query, its caller's reply waits here,
//! and the serve loop finishes it after a poll, once the answer is in or its
//! deadline has passed. Kept free of smoltcp and the kernel so the host proofs
//! include it unchanged.

pub use super::waiting::{Look, Waiting, PENDING_MAX};

pub struct Table<Q> {
    slots: [Option<Waiting<Q>>; PENDING_MAX],
}

impl<Q: Copy> Table<Q> {
    pub const fn new() -> Self {
        Self { slots: [None; PENDING_MAX] }
    }

    /// Whether any lookup is waiting, so the loop keeps polling for it.
    pub fn any(&self) -> bool {
        self.slots.iter().any(Option::is_some)
    }

    /// Keep a started lookup. `Err` gives it back when every slot is taken, for
    /// the caller to be answered at once.
    pub fn add(&mut self, w: Waiting<Q>) -> Result<(), Waiting<Q>> {
        match self.slots.iter_mut().find(|s| s.is_none()) {
            Some(slot) => {
                *slot = Some(w);
                Ok(())
            }
            None => Err(w),
        }
    }

    /// Remove and return the lookup `pid` is owed an answer for, if any.
    pub fn take_for(&mut self, pid: u32) -> Option<Waiting<Q>> {
        self.slots.iter_mut().find(|s| s.is_some_and(|w| w.pid == pid))?.take()
    }

    /// Look at every waiting lookup once. `look(w, expired)` answers the caller
    /// when it can and says so, and may note what it read in `w`; a lookup
    /// whose deadline has passed is removed whatever `look` says.
    pub fn sweep(&mut self, now_ms: i64, mut look: impl FnMut(&mut Waiting<Q>, bool) -> Look) {
        for slot in self.slots.iter_mut() {
            let Some(w) = slot.as_mut() else { continue };
            let expired = now_ms >= w.deadline_ms;
            if look(w, expired) == Look::Answered || expired {
                *slot = None;
            }
        }
    }
}

impl<Q: Copy> Default for Table<Q> {
    fn default() -> Self {
        Self::new()
    }
}
