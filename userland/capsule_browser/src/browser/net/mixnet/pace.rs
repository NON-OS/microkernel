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

//! How often the proxies are asked, so the window is never held long.
//!
//! Every call to a proxy is made on the thread that draws the window and
//! answers the pointer, and each may wait the poll wait. Two bounds keep
//! that short. A tick spends at most TICK_BUDGET_MS in calls, and the call
//! that crosses it is the last; what is left is asked on the next tick. And
//! a proxy that let a call go unanswered is not asked again, by any
//! conversation, for REASK_GAP_MS: it is busy (net.socks5 opening a mixnet
//! session waits on net.nym for many seconds), so asking each tick would
//! hold every tick for the whole wait and pile up calls the kernel keeps a
//! place for until the proxy gets to them, eight per caller at most.
//!
//! Pure; the proofs hold it.

use alloc::vec::Vec;

/// What any call to a proxy may wait for, sending or not.
///
/// Every call happens on the thread that draws the window and answers the
/// pointer. Waiting longer for an answer that has not arrived stops the
/// whole application: the reader cannot move the window, reach a menu or
/// stop the page, and the browser looks like it has crashed when it is in
/// fact waiting patiently. Bytes for the exit used to wait 15 s, and then
/// up to 75 s over several calls while a cold mixnet session opened. A
/// frame left unanswered now stays asked, and later ticks ask it again
/// under the same number (`conv`).
pub const POLL_MS: u64 = 60;

/// What a tick may spend waiting on proxies before it lets the window draw.
pub const TICK_BUDGET_MS: i64 = 45;

/// How long a proxy that left a call unanswered is left alone.
pub const REASK_GAP_MS: i64 = 200;

/// A budget never renewed by a tick is renewed after this, so a caller that
/// forgot to begin its tick is slowed, not stopped.
const STALE_MS: i64 = 1_000;

pub struct Pace {
    /// Proxies that left a call unanswered, and when they may be asked.
    quiet: Vec<(u32, i64)>,
    /// Milliseconds spent in calls since `since`.
    spent: i64,
    since: i64,
}

impl Pace {
    pub const fn new() -> Pace {
        Pace { quiet: Vec::new(), spent: 0, since: i64::MIN }
    }

    /// A tick begins at `now` with its whole budget.
    pub fn new_tick(&mut self, now: i64) {
        self.spent = 0;
        self.since = now;
    }

    /// Whether `port` may be asked at `now`.
    pub fn may_ask(&mut self, port: u32, now: i64) -> bool {
        if self.since == i64::MIN || now.saturating_sub(self.since) >= STALE_MS {
            self.new_tick(now);
        }
        let rested = self.quiet.iter().all(|&(p, until)| p != port || now >= until);
        self.spent < TICK_BUDGET_MS && rested
    }

    /// A call to `port` ran from `from` to `to`, answered or not.
    pub fn asked(&mut self, port: u32, from: i64, to: i64, answered: bool) {
        self.spent = self.spent.saturating_add(to.saturating_sub(from).max(0));
        self.quiet.retain(|&(p, _)| p != port);
        if !answered {
            self.quiet.push((port, to.saturating_add(REASK_GAP_MS)));
        }
    }
}

impl Default for Pace {
    fn default() -> Self {
        Self::new()
    }
}
