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
//! What a rejoin pass, and a join it hands the driver, lead to.

use super::{Machine, Phase, Step, JOIN_WATCH_MS, REJOIN_MAX_MS, WATCH_EVERY_MS};

impl Machine {
    pub(super) fn after_rejoin(&mut self, now_ms: i64, step: Step) {
        match step {
            Step::Answered { joined: true, .. } => self.joined(),
            Step::Started(index) => {
                let until_ms = now_ms.saturating_add(JOIN_WATCH_MS);
                self.phase = Phase::Joining { index, until_ms };
                self.next_ms = now_ms.saturating_add(WATCH_EVERY_MS);
            }
            Step::Refused(_) | Step::Done => self.give_up(),
            Step::NotYet | Step::NoneInRange | Step::Answered { joined: false, .. } => {
                self.link.left = self.link.left.saturating_sub(1);
                if self.link.left == 0 {
                    return self.give_up();
                }
                self.link.backoff_ms = self.link.backoff_ms.saturating_mul(2).min(REJOIN_MAX_MS);
                self.next_ms = now_ms.saturating_add(self.link.backoff_ms);
            }
        }
    }

    /// A join came up: autojoin is done and the link is watched from here.
    pub(super) fn joined(&mut self) {
        self.phase = Phase::Done;
        self.link.left = 0;
        self.link.up = true;
    }

    /// The rejoin ends without the link. Only a link seen up again, joined
    /// from Settings, starts another.
    pub(super) fn give_up(&mut self) {
        self.phase = Phase::Done;
        self.link.left = 0;
    }
}
