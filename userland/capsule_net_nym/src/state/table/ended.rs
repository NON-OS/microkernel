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
 * Only a session's owner can close it, so a client that ended without
 * closing left each of its sessions here for good: a place in the table, a
 * session key, and up to RX_BYTES_MAX of replies nobody would read. A
 * directory refresh also waits for the table to be empty, so one such
 * session held the directory at its age until a reboot. These are closed
 * as their owner's own close would have closed them.
 */

use super::types::Table;

impl Table {
    /// How many sessions `owner` holds.
    pub fn held_by(&self, owner: u32) -> usize {
        self.sessions.iter().filter(|s| s.owner == owner).count()
    }

    /// Close every session whose owner `alive` says has ended, and say how
    /// many were closed.
    pub fn close_ended(&mut self, alive: impl Fn(u32) -> bool) -> usize {
        let before = self.sessions.len();
        self.sessions.retain_mut(|session| {
            if alive(session.owner) {
                return true;
            }
            session.zeroize();
            false
        });
        before.saturating_sub(self.sessions.len())
    }
}
