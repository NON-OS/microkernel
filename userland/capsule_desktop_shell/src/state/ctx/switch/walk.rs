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

//! A press of Alt+Tab, and the run of presses ending.

use super::order::{SwitchOrder, Win};
use super::session::Session;

impl SwitchOrder {
    /// A press at uptime `now_ms`: the window to focus, the next one (the one
    /// before with `back`) in the order as it stood when the run began. A
    /// press after the run's hold starts a new run, and so does one after a
    /// run with nothing to switch to, so a window opened since is in it. None
    /// with fewer than two windows.
    pub fn step(&mut self, now_ms: i64, back: bool) -> Option<Win> {
        let walking = self.session.as_ref().is_some_and(|s| s.live(now_ms) && s.windows.len() > 1);
        if !walking {
            self.session = Some(Session::new(self.recent.clone(), now_ms));
        }
        self.session.as_mut()?.step(now_ms, back)
    }

    /// End a run whose hold is over; whether one ended, so the strip goes.
    pub fn expire(&mut self, now_ms: i64) -> bool {
        let over = self.session.as_ref().is_some_and(|s| !s.live(now_ms));
        if over {
            self.session = None;
        }
        over
    }
}
