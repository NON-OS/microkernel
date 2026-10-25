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

//! After a failure: the rest before the next restart, or the radio down
//! once the tries are spent.

use crate::firmware::gen3::outcome::Failure;

use super::restart::Restart;
use super::say::Line;
use super::state::{Radio, State};

impl Radio {
    /// Wait for the next restart, or stay down with `why`.
    pub(super) fn wait_or_stay_down(&mut self, mut r: Restart, why: Failure, now: u64) {
        Line::new().text(b"radio down: ").text(why.text().as_bytes()).send();
        match r.restarts.after_failure(now) {
            Some(at_ms) => {
                let n = u32::from(r.restarts.tries());
                Line::new().text(b"firmware restart ").num(n).text(b" after a rest").send();
                r.at_ms = at_ms;
                self.state = State::Restarting(r);
            }
            None => {
                Line::new().text(b"firmware restarts spent; the radio stays down").send();
                self.state = State::Down(why);
            }
        }
    }
}
