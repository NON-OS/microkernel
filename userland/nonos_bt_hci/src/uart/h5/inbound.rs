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

//! A link control message heard. SLEEP and WOKEN tell whether the peer
//! can take packets now (Vol 4, Part D, "Low Power"); the rest go to link
//! establishment. When the peer has started over, this side starts over
//! too, then answers the SYNC that said so as a side starting over does.

use super::control::Message;
use super::engine::H5;
use super::heard::Heard;

impl H5 {
    /// Reliable packets sent or queued and not yet acknowledged.
    pub fn unacked(&self) -> usize {
        self.rel.unacked.len()
    }

    /// The peer has stopped acknowledging.
    pub fn lost(&self, now: u64) -> bool {
        self.rel.lost(now)
    }

    pub(super) fn control(&mut self, m: Message) {
        match m {
            Message::Sleep => return self.peer_asleep = true,
            Message::Woken | Message::Wakeup => self.peer_asleep = false,
            _ => {}
        }
        let mut answer = self.link.heard(m);
        if answer == Heard::Restart {
            let restarts = self.restarts.saturating_add(1);
            *self = Self { restarts, ..Self::default() };
            answer = self.link.heard(m);
        }
        if let Heard::Reply(r) = answer {
            self.replies.push_back(r);
        }
        self.deslip.oof = self.active() && self.link.agreed.oof;
    }
}
