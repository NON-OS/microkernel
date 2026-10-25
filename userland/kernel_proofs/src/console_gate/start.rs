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

//! The world a run starts in, and when it has finished.

use super::actors::{Hold, Taker, World};
use super::drawer::fresh;
use super::gate::Gate;
use super::world::{At, Flags};

impl World {
    /// Two drawers trying `draws` draws each, `takers` takeovers, the hold.
    pub fn new(g: &Gate, draws: u8, takers: usize, hold: bool) -> World {
        let taker = |i| if i < takers { Taker::Start } else { Taker::Off };
        World {
            flags: Flags::default(),
            drawers: [fresh(g, draws - 1); 2],
            takers: [taker(0), taker(1)],
            hold: if hold { Hold::Start } else { Hold::Off },
        }
    }

    pub fn done(&self) -> bool {
        self.drawers.iter().all(|d| d.at == At::Done)
            && self.takers.iter().all(|t| matches!(t, Taker::Off | Taker::Ended { .. }))
            && matches!(self.hold, Hold::Off | Hold::Cleared)
    }
}
