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

//! The gate's rules, apart from where its state is kept, so they are
//! checked on the host with a clock of their own.

/// Calls in a row that may go unanswered before the gate closes.
pub const UNANSWERED_LIMIT: u32 = 3;
pub const FIRST_PAUSE_MS: u64 = 1_000;
pub const MAX_PAUSE_MS: u64 = 30_000;

/// The gate's state, apart from where it is kept, so the rules can be
/// checked on the host with a clock of their own.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gate {
    pub unanswered: u32,
    pub pauses: u32,
    pub closed_until_ms: u64,
}

/// The pause after `pauses` earlier ones: 1 s, 2 s, 4 s ... held at 30 s.
pub const fn pause_ms(pauses: u32) -> u64 {
    let shift = if pauses > 5 { 5 } else { pauses };
    let ms = FIRST_PAUSE_MS << shift;
    if ms > MAX_PAUSE_MS {
        MAX_PAUSE_MS
    } else {
        ms
    }
}

impl Gate {
    pub const fn open(&self, now_ms: u64) -> bool {
        now_ms >= self.closed_until_ms
    }

    /// Count one call's result: `got` is what `mk_ipc_call_timeout`
    /// returned, negative when no reply came back at all.
    pub fn record(&mut self, got: i64, now_ms: u64) {
        if got >= 0 {
            *self = Gate::default();
            return;
        }
        self.unanswered += 1;
        if self.unanswered >= UNANSWERED_LIMIT {
            self.closed_until_ms = now_ms.saturating_add(pause_ms(self.pauses));
            self.pauses = self.pauses.saturating_add(1);
            self.unanswered = 0;
        }
    }
}
