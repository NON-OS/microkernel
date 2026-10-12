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

//! What the radio says about itself.

use crate::firmware::gen3::outcome::STAGE_READY;

use super::state::{Radio, State};

impl Radio {
    /// The firmware is up and scanning.
    pub fn scanning(&self) -> bool {
        matches!(self.state, State::Up(_))
    }

    /// This path has touched the card, so nothing else may drive it.
    pub fn owns_card(&self) -> bool {
        self.seen.touched
    }

    /// The stage code, step and detail word for the status reply.
    pub fn stage(&self) -> (u8, u8, u32) {
        match &self.state {
            State::Up(_) => (STAGE_READY, 0, 0),
            State::Down(f) => (f.stage(), f.step(), f.detail()),
        }
    }

    pub fn hw_rev(&self) -> u32 {
        self.seen.hw_rev
    }

    pub fn rf_id(&self) -> u32 {
        self.seen.rf_id
    }

    /// Sweeps completed, sweeps stalled, and frames received.
    pub fn counts(&self) -> (u32, u32, u32) {
        (self.sweep.completed, self.sweep.stalled, self.sweep.frames)
    }
}
