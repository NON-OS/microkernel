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

//! A channel to the core that was up and went down: the core is launched
//! again and its endpoint looked for, as at the first start. A core that is
//! still running answers the launch busy and is reconnected to; one that
//! ended is started again with the profile it already has. A core that
//! closes MAX_LOSSES channels in a row with no frame between them is
//! stopped on, so one that dies at once is not started without end.

use super::start::{Phase, Start};

pub const MAX_LOSSES: u8 = 3;

/// What the window says when it stops starting the core again.
pub const KEEPS_CLOSING: &str = "the core keeps closing the channel";

impl Start {
    /// The channel that was up is gone, at `now`. The next step waits a whole
    /// EVERY_MS, as after any launch.
    pub fn lost(&mut self, now: i64) {
        if self.phase != Phase::Up {
            return;
        }
        self.losses = self.losses.saturating_add(1);
        self.last = now;
        self.again = true;
        (self.probe, self.starts) = (false, 0);
        self.phase = match self.losses > MAX_LOSSES {
            true => Phase::Stopped(KEEPS_CLOSING),
            false => Phase::Launch(None),
        };
    }

    /// A frame came from the core: the channel works, and losses before it
    /// no longer count.
    pub fn steady(&mut self) {
        self.losses = 0;
    }
}
