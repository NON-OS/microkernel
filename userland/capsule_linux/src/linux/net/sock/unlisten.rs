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

//! A listener that stops: what it queued is reset, and the connects that
//! wait for its room are refused.

use crate::linux::abi::errno::ECONNREFUSED;

use super::table::Socks;

impl Socks {
    /// Listener `l` is gone: the connects waiting on it are refused.
    pub fn refuse_waiting(&mut self, syn: impl Iterator<Item = u32>) {
        for c in syn {
            if let Some(cs) = self.get_mut(c) {
                cs.connecting = false;
                cs.error = ECONNREFUSED;
            }
        }
    }

    /// Listener `l` stops listening: what it queued is reset, and the
    /// connects waiting for room are refused.
    pub fn unlisten(&mut self, l: u32) {
        let Some(s) = self.get_mut(l) else {
            return;
        };
        s.listening = false;
        let (queued, waiting) = (core::mem::take(&mut s.pending), core::mem::take(&mut s.syn));
        for q in queued {
            self.free(q, true);
        }
        self.refuse_waiting(waiting.into_iter());
    }
}
