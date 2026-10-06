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

/*
 * Connections a SYN opened and the peer has not finished. Each one's entry
 * counts against the listener owner's connections, and every application's
 * sockets reach net.tcp through net.sockets as one owner, so half-open
 * entries that never ended let a handful of forged SYNs refuse every
 * connection on the machine, outgoing ones included. A listener therefore
 * holds at most HALF_OPEN_MAX of them, a SYN past that takes the place of
 * the oldest, and each ends HALF_OPEN_MS after its SYN unless the peer
 * finished it. A peer that answers within the window still gets in under a
 * flood, and forged SYNs only ever hold this listener's few places.
 */

use super::types::Table;
use crate::state::TimerKind;
use crate::tcp::{State, HALF_OPEN_MAX, HALF_OPEN_MS};

impl Table {
    /// The half-open connections under `parent`, and the oldest of them.
    fn half_open_of(&self, parent: u32) -> (usize, Option<u32>) {
        let mut count = 0usize;
        let mut oldest: Option<(u64, u32)> = None;
        for e in self.entries.iter() {
            if e.parent != parent || e.tcb.state != State::SynReceived {
                continue;
            }
            count += 1;
            let ends = self.timers.deadline_of(e.handle, TimerKind::HalfOpen).unwrap_or(0);
            if oldest.is_none_or(|(d, _)| ends < d) {
                oldest = Some((ends, e.handle));
            }
        }
        (count, oldest.map(|(_, h)| h))
    }

    /// Drop the oldest half-open connection under `parent` while it holds
    /// HALF_OPEN_MAX, so one more SYN can be taken.
    pub fn make_half_open_room(&mut self, parent: u32) {
        while let (count, Some(oldest)) = self.half_open_of(parent) {
            if count < HALF_OPEN_MAX {
                return;
            }
            self.remove_by_handle(oldest);
            self.timers.cancel_all(oldest);
        }
    }

    /// Start the time a half-open connection has to be finished.
    pub fn arm_half_open(&mut self, handle: u32, now_ms: u64) {
        self.timers.arm(handle, TimerKind::HalfOpen, now_ms.saturating_add(HALF_OPEN_MS));
    }

    /// End `handle` if it is still half-open; one the peer finished stays.
    pub fn expire_half_open(&mut self, handle: u32) {
        let unfinished =
            self.entries.iter().any(|e| e.handle == handle && e.tcb.state == State::SynReceived);
        if unfinished {
            self.remove_by_handle(handle);
            self.timers.cancel_all(handle);
        }
    }
}
