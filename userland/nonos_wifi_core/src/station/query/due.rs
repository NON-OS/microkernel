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

//! When the next SA Query request is due, and when the query has run out.

use super::state::{Query, MAX_MS, RETRY_MS};

/// What the query calls for now.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(in crate::station) enum Asked {
    Nothing,
    /// Send a request with this identifier.
    Ask(u16),
    /// No answer in time: the deauthentication with this reason stands.
    Unanswered(u16),
}

impl Query {
    pub(in crate::station) fn due(&mut self, now: u64) -> Asked {
        let Some(mut a) = self.asking else {
            return Asked::Nothing;
        };
        let Some(started) = a.started else {
            // Begun before the clock ran: it starts now.
            a.started = Some(now);
            a.next_ms = now.saturating_add(RETRY_MS);
            self.asking = Some(a);
            return Asked::Nothing;
        };
        if now.saturating_sub(started) >= MAX_MS {
            self.asking = None;
            return Asked::Unanswered(a.reason);
        }
        if now < a.next_ms {
            return Asked::Nothing;
        }
        let id = self.take_id();
        a.sent = a.sent.saturating_add(1);
        a.next_ms = now.saturating_add(RETRY_MS);
        self.asking = Some(a);
        Asked::Ask(id)
    }

    /// A fresh transaction identifier.
    pub(super) fn take_id(&mut self) -> u16 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        id
    }
}
