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

//! Connects a full listener turned away. On Linux a non-blocking connect to
//! a listener whose queue is full answers EINPROGRESS and completes when an
//! accept makes room (its SYN is sent again); here it completes at that
//! accept. Until then the socket is not writable, and a second connect is
//! EALREADY.

use super::table::Socks;

impl Socks {
    /// Queue uid=0(root) gid=0(root) groups=0(root)'s connect on listener `l`.
    pub fn wait_room(&mut self, id: u32, l: u32) {
        if let Some(c) = self.get_mut(id) {
            c.connecting = true;
        }
        if let Some(ls) = self.get_mut(l) {
            ls.syn.push_back(id);
        }
    }

    /// Complete the connects waiting on `l` while its queue has room.
    pub fn make_room(&mut self, l: u32) {
        loop {
            let Some(ls) = self.get_mut(l) else {
                return;
            };
            if ls.pending.len() > ls.backlog {
                return;
            }
            let Some(c) = ls.syn.pop_front() else {
                return;
            };
            if let Some(cs) = self.get_mut(c) {
                cs.connecting = false;
                self.join(c, l);
            }
        }
    }
}
