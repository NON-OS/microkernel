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

//! When to read the batteries again before the minute's poll: while a
//! battery's _STA faulted (its EC not answering yet, or on trial after a
//! stall), or one present lacks its information or status. Back-off from
//! 250 ms, doubling, eight times: about a minute in all, after which the
//! minute's poll carries on. A battery that answered, present or not,
//! needs nothing. Pure, proved in nonos_aml_proofs.

use super::state::Power;

pub const FIRST_MS: u64 = 250;
pub const TRIES: u8 = 8;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Retry {
    tries: u8,
    due_ms: Option<u64>,
}

/// Whether any reading is still owed.
pub fn wanted(p: &Power) -> bool {
    p.batteries.iter().any(|b| b.unread || (b.present && (b.info.is_none() || b.status.is_none())))
}

impl Retry {
    /// After a reading at `now_ms`: the next one due, or none.
    pub fn after(&mut self, wanted: bool, now_ms: u64) {
        if !wanted {
            *self = Retry::default();
            return;
        }
        if self.tries >= TRIES {
            self.due_ms = None;
            return;
        }
        self.due_ms = Some(now_ms.saturating_add(FIRST_MS << self.tries));
        self.tries += 1;
    }

    pub fn due(&self, now_ms: u64) -> bool {
        self.due_ms.is_some_and(|at| now_ms >= at)
    }
}
