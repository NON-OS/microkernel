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
 * Connections whose owner ended without closing them. Only the owner can
 * close or read one, so each stayed for good, holding a place in the table
 * and leaving its peer with an open connection nobody answers. They are
 * taken out once their owner no longer runs; the caller resets those the
 * peer still holds open. TIME-WAIT is left to end by itself.
 */

use alloc::vec::Vec;

use super::types::Table;
use crate::tcp::{State, Tcb};

impl Table {
    /// Take out every entry whose owner `alive` says has ended, and return
    /// the connections a peer still holds open.
    pub fn take_orphans(&mut self, alive: impl Fn(u32) -> bool) -> Vec<Tcb> {
        let mut open = Vec::new();
        let mut gone = Vec::new();
        for e in self.entries.iter() {
            if e.tcb.state == State::TimeWait || alive(e.owner_pid) {
                continue;
            }
            gone.push(e.handle);
            if !matches!(e.tcb.state, State::Listen | State::SynSent) {
                open.push(e.tcb);
            }
        }
        for h in gone {
            self.remove_by_handle(h);
            self.timers.cancel_all(h);
        }
        open
    }
}
