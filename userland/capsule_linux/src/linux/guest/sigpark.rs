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

//! Which wait a thread is parked in, and taking it out of every one, so a
//! thread that has gone leaves no wait behind to be answered.

use super::handle::Guest;

/// Parked, and in which kind of wait.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Parked {
    /// A timed sleep, with its monotonic deadline.
    Sleep(u64),
    /// Any other wait a signal ends.
    Wait,
}

impl Guest {
    /// The wait `tid` is parked in, if a signal can end it.
    pub fn parked(&self, tid: u32) -> Option<Parked> {
        if let Some(&(due, _)) = self.sleepers.iter().find(|(_, t)| *t == tid) {
            return Some(Parked::Sleep(due));
        }
        let waiting = self.waits.iter().any(|(t, _)| *t == tid)
            || self.pipe_wait.is_some_and(|w| w.3 == tid)
            || self.signals.sigwaits.iter().any(|w| w.tid == tid)
            || self.signals.childwaits.iter().any(|w| w.tid == tid);
        waiting.then_some(Parked::Wait)
    }

    /// Take `tid` out of every wait it is parked in, without answering it.
    pub fn leave_waits(&mut self, tid: u32) -> Option<Parked> {
        let was = self.parked(tid)?;
        self.sleepers.retain(|(_, t)| *t != tid);
        self.waits.retain(|(t, _)| *t != tid);
        if self.pipe_wait.is_some_and(|w| w.3 == tid) {
            self.pipe_wait = None;
        }
        self.signals.sigwaits.retain(|w| w.tid != tid);
        self.signals.childwaits.retain(|w| w.tid != tid);
        Some(was)
    }
}
