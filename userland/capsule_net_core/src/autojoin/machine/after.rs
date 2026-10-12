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

//! What each pass and each look at the link lead to.

use super::state::bit;
use super::{
    Machine, Phase, Step, EMPTY_PASSES_MAX, JOIN_WATCH_MS, NOT_YET_MS, RESCAN_MS, WATCH_EVERY_MS,
};

impl Machine {
    pub fn after_attempt(&mut self, now_ms: i64, step: Step) {
        let wait = match step {
            Step::NotYet => NOT_YET_MS,
            Step::Answered { joined: true, .. } | Step::Done => {
                self.phase = Phase::Done;
                return;
            }
            Step::Answered { index, joined: false } => {
                self.tried |= bit(index);
                NOT_YET_MS
            }
            Step::Started(index) => {
                self.tried |= bit(index);
                self.phase = Phase::Joining { index, until_ms: now_ms + JOIN_WATCH_MS };
                WATCH_EVERY_MS
            }
            Step::NoneInRange => {
                self.empty_passes = self.empty_passes.saturating_add(1);
                if self.empty_passes >= EMPTY_PASSES_MAX {
                    self.phase = Phase::Done;
                    return;
                }
                RESCAN_MS
            }
        };
        self.next_ms = now_ms + wait;
    }

    /// What the link said while a join runs: `Some(true)` associated,
    /// `Some(false)` not (yet), `None` no answer in time (the driver is busy
    /// joining). Returns the index of a join that just ended, and whether it
    /// joined, for the log.
    pub fn after_watch(&mut self, now_ms: i64, associated: Option<bool>) -> Option<(u8, bool)> {
        let Phase::Joining { index, until_ms } = self.phase else { return None };
        if associated == Some(true) {
            self.phase = Phase::Done;
            return Some((index, true));
        }
        if now_ms >= until_ms {
            self.phase = Phase::Waiting;
            self.next_ms = now_ms + NOT_YET_MS;
            return Some((index, false));
        }
        self.next_ms = now_ms + WATCH_EVERY_MS;
        None
    }
}
