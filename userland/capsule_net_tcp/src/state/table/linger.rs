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
 * Entries an application has closed and the stack keeps for the peer's
 * sake. Every application's sockets reach net.tcp as one owner, so these
 * must not hold that owner's places. TIME-WAIT absorbs a late segment of the
 * old connection for twice the MSL; it is not counted against the owner, and
 * when the table is full the oldest gives its place, as Linux does past
 * tcp_max_tw_buckets. FIN-WAIT-2 waits for a FIN the peer may never send
 * (one that crashed, or a NAT that forgot it); it ends FIN_WAIT_2_MS after
 * our FIN was acknowledged, Linux's tcp_fin_timeout, unless the FIN came.
 */

use super::types::Table;
use crate::state::TimerKind;
use crate::tcp::State;

impl Table {
    /// Free the oldest TIME-WAIT entry; false when there is none.
    pub(super) fn reclaim_time_wait(&mut self) -> bool {
        let mut oldest: Option<(u64, u32)> = None;
        for e in self.entries.iter().filter(|e| e.tcb.state == State::TimeWait) {
            let ends = self.timers.deadline_of(e.handle, TimerKind::TimeWait).unwrap_or(0);
            if oldest.is_none_or(|(d, _)| ends < d) {
                oldest = Some((ends, e.handle));
            }
        }
        let Some((_, handle)) = oldest else { return false };
        self.remove_by_handle(handle);
        self.timers.cancel_all(handle);
        true
    }

    /// End `handle` if its peer never sent a FIN; one in TIME-WAIT stays.
    pub fn expire_fin_wait_2(&mut self, handle: u32) {
        let waiting =
            self.entries.iter().any(|e| e.handle == handle && e.tcb.state == State::FinWait2);
        if waiting {
            self.remove_by_handle(handle);
            self.timers.cancel_all(handle);
        }
    }
}
