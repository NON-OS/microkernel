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

//! What each pass leads to.

use super::state::bit;
use super::{
    Machine, Phase, Step, EMPTY_PASSES_MAX, JOIN_WATCH_MS, NOT_YET_MS, REJOIN_ATTEMPTS,
    REJOIN_FIRST_MS, REJOIN_MAX_MS, RESCAN_MS, WATCH_EVERY_MS,
};

impl Machine {
    pub fn after_attempt(&mut self, now_ms: i64, step: Step) {
        if self.rejoining() {
            return self.after_rejoin(now_ms, step);
        }
        let wait = match step {
            Step::NotYet => NOT_YET_MS,
            Step::Answered { joined: true, .. } => return self.joined(),
            // Every saved network was tried this round: one that failed for
            // a reason anyone on the air could cause is tried again, after
            // twice the wait of the round before, for a bounded number.
            Step::Done if self.tried & !self.refused == 0 || self.rounds >= REJOIN_ATTEMPTS => {
                self.phase = Phase::Done;
                return;
            }
            Step::Done => {
                self.rounds += 1;
                self.tried = self.refused;
                REJOIN_FIRST_MS.saturating_mul(1 << self.rounds.min(15)).min(REJOIN_MAX_MS)
            }
            Step::Refused(index) => {
                self.tried |= bit(index);
                self.refused |= bit(index);
                NOT_YET_MS
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
}
