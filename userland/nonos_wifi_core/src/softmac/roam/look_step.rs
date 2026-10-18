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

//! One step of a look: at home until it is time to go and the link is in
//! no exchange that must see it through, away a dwell on each channel of a
//! trip, then home again, until the plan is done.

use super::look::{Look, Phase, Step, AWAY_CHANNELS, HOME_MS};
use crate::softmac::radio::Radio;

impl Look {
    /// Off the session's channel: the link's frames must not go out.
    pub fn away(&self) -> bool {
        matches!(self.phase, Phase::Away { .. })
    }

    /// One step; while `hold`, a trip waits at home to start.
    pub fn step<R: Radio>(&mut self, r: &mut R, ssid: &[u8], now: u64, hold: bool) -> Step {
        match self.phase {
            Phase::Done => Step::Done,
            Phase::Home { until_ms } if now < until_ms || hold => Step::Home,
            Phase::Home { .. } => match self.next_channel() {
                Some(at) => {
                    self.leave(r, at, AWAY_CHANNELS - 1, now);
                    Step::Away
                }
                None => {
                    self.phase = Phase::Done;
                    Step::Done
                }
            },
            Phase::Away { left, until_ms, at } => {
                self.listen(r, ssid, at);
                if now < until_ms {
                    return Step::Away;
                }
                // The trip's last channel draws nothing from the plan: the next
                // channel is the next trip's first.
                let next = if left > 0 { self.next_channel() } else { None };
                match next {
                    Some(next) => {
                        self.leave_to(r, next, left - 1, now);
                        Step::Away
                    }
                    None => {
                        self.come_home(r);
                        self.phase = Phase::Home { until_ms: now.saturating_add(HOME_MS) };
                        Step::Home
                    }
                }
            }
        }
    }
}
