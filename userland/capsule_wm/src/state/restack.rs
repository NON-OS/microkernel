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

//! Whether the compositor still owes this stack a restack, and when to send
//! it. Pure; held in wm_proofs.
//!
//! Every raise and focus change here is told to the compositor with a
//! focus_set, which lifts that process's layer. The compositor reads requests
//! between frames, and a frame it composes whole can take far longer than the
//! call's 16 ms: "ipc.call unanswered compositor caller=wm". Such a call was
//! still delivered, the compositor acts on it in order, and asking again
//! would only lift the same layer twice, so a late answer counts as done.
//!
//! A call the compositor never received (refused, or no compositor to take
//! it) is different: the screen keeps the old stacking while this table has
//! the new one, and a press goes to a window drawn under another. Nothing
//! told it again. Now such a loss marks a restack owed, and the service loop
//! sends the whole stack, bottom to top, once it is due, at a doubling
//! interval while the compositor still does not take it. The serve loop is
//! never held for more than one pass over the open windows.

/// What became of one call to the compositor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// It answered.
    Answered,
    /// It took the request but its answer came too late.
    Late,
    /// It never took the request.
    Lost,
}

impl Outcome {
    pub fn delivered(self) -> bool {
        self != Outcome::Lost
    }
}

/// The wait before the first resend.
pub const FIRST_WAIT_MS: i64 = 50;
/// The longest wait between resends.
pub const MAX_WAIT_MS: i64 = 4000;

pub struct Restack {
    owed: bool,
    at_ms: i64,
    wait_ms: i64,
}

impl Default for Restack {
    fn default() -> Self {
        Self::new()
    }
}

impl Restack {
    pub const fn new() -> Self {
        Self { owed: false, at_ms: 0, wait_ms: FIRST_WAIT_MS }
    }

    /// One call's outcome, at uptime `now_ms`. A loss owes a restack; a
    /// delivered call leaves an owed one owed, since one lift does not set
    /// the rest of the order right.
    pub fn note(&mut self, outcome: Outcome, now_ms: i64) {
        if !outcome.delivered() && !self.owed {
            self.owed = true;
            self.at_ms = now_ms.saturating_add(self.wait_ms);
        }
    }

    /// Whether the restack is owed and its time has come.
    pub fn due(&self, now_ms: i64) -> bool {
        self.owed && now_ms >= self.at_ms
    }

    pub fn owed(&self) -> bool {
        self.owed
    }

    /// The whole stack went through: nothing is owed and the next loss
    /// waits the shortest again.
    pub fn done(&mut self) {
        *self = Self::new();
    }

    /// The restack was lost too: try again after twice the wait, up to
    /// `MAX_WAIT_MS`, never giving up.
    pub fn retry(&mut self, now_ms: i64) {
        self.wait_ms = self.wait_ms.saturating_mul(2).min(MAX_WAIT_MS);
        self.at_ms = now_ms.saturating_add(self.wait_ms);
    }
}
