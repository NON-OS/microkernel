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

//! What a job tells the exit rule and STATUS, and what an owner's frame
//! changes in it (state.rs).

use nonos_proof_abi::wire::Stage;

use super::exit_rule::Seen;
use super::state::Job;

impl Job {
    /// What the exit rule reads of it.
    pub fn seen(&self) -> Seen {
        let (stage, fetched, told, asker_ms) = (self.stage, self.fetched, self.told, self.asker_ms);
        Seen { stage, fetched, told, asker_ms, working: self.working }
    }

    /// The owner sent a frame at `now`. A frame after an answered failure
    /// means the call that carried it has returned, so the failure is read;
    /// had that call given up first, this frame is answered with it again.
    pub fn heard(&mut self, now: u64) {
        self.told |= self.answered;
        self.asker_ms = now;
    }

    pub fn total(&self) -> u32 {
        if self.stage == Stage::Done {
            self.out.len() as u32
        } else {
            0
        }
    }
}
