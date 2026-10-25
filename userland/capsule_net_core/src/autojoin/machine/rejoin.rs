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
//! Joining the network again after a link that was up was lost.
//!
//! Once autojoin is done, `tick` looks at the link every `LINK_EVERY_MS`. A
//! link seen up and then down starts a rejoin of that network alone, unless
//! the person left it (a Leave from Settings or the setup wizard): at most
//! `REJOIN_ATTEMPTS` passes, the first `REJOIN_FIRST_MS` after the loss, each
//! later wait twice the one before, up to `REJOIN_MAX_MS`. A join refused for
//! good ends it (`codes`: what no one on the air can forge), and so does one
//! the driver ran on whose result never came back.

use super::seen::Rejoin;
use super::{Machine, Phase, LINK_EVERY_MS, REJOIN_ATTEMPTS, REJOIN_FIRST_MS};

impl Machine {
    /// Autojoin is done and the link is due a look.
    pub fn link_due(&self, now_ms: i64) -> bool {
        self.phase == Phase::Done && now_ms >= self.link.next_ms
    }

    /// A rejoin is under way: each pass is for the network that was up.
    pub fn rejoining(&self) -> bool {
        self.link.left > 0
    }

    /// What the link said: whether it is associated, or `None` when the
    /// driver did not answer in time, and whether the person left it. True
    /// when this starts a rejoin.
    pub fn after_link(&mut self, now_ms: i64, associated: Option<bool>, left: bool) -> bool {
        if self.phase != Phase::Done {
            return false;
        }
        let next_ms = now_ms.saturating_add(LINK_EVERY_MS);
        self.link.next_ms = next_ms;
        match associated {
            Some(true) => self.link.up = true,
            Some(false) if self.link.up && !left => {
                let backoff_ms = REJOIN_FIRST_MS;
                self.link = Rejoin { up: false, left: REJOIN_ATTEMPTS, backoff_ms, next_ms };
                self.phase = Phase::Waiting;
                self.next_ms = now_ms.saturating_add(backoff_ms);
                return true;
            }
            Some(false) => self.link.up = false,
            None => {}
        }
        false
    }
}
