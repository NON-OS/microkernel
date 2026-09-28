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

//! Letting a socket go, and what its peer sees when it does.

use crate::linux::abi::errno::ECONNRESET;

use super::table::Socks;

impl Socks {
    /// `pid` no longer holds `id`. The socket goes when nobody does.
    pub fn release(&mut self, id: u32, pid: u32) {
        let Some(s) = self.get_mut(id) else {
            return;
        };
        let held = s.holders.len();
        s.holders.retain(|&p| p != pid);
        if held != 0 && s.holders.is_empty() {
            self.free(id, false);
        }
    }

    /// Close `id`. Its peer reads end of file, or ECONNRESET if this end
    /// left bytes unread, set SO_LINGER to zero seconds, or `reset` is set,
    /// which is when Linux sends a reset instead of a FIN. Connections still
    /// queued on a listener are reset.
    pub fn free(&mut self, id: u32, reset: bool) {
        let Some(gone) = self.list.get_mut(id as usize).and_then(Option::take) else {
            return;
        };
        if let Some(h) = gone.svc {
            super::super::stream::close(h);
        }
        if let Some(p) = gone.peer.and_then(|p| self.get_mut(p)) {
            p.peer = None;
            p.eof = true;
            if reset || !gone.rx.is_empty() || gone.opts.linger == (1, 0) {
                p.error = ECONNRESET;
            }
        }
        for queued in gone.pending {
            self.free(queued, true);
        }
    }
}
