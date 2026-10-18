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

//! The ATT transaction timer, failing, and which request a response
//! answers.

use super::phase::Phase;
use super::state::Host;
use super::step::{Outcome, Step};
use crate::error::HogpError;

impl Host {
    /// Whether discovery's request has gone unanswered too long; if so the
    /// host has failed, and the step says so.
    pub fn poll(&mut self, now_ms: u64) -> Step {
        if self.expired(now_ms) {
            self.fail(HogpError::Timeout)
        } else {
            Step::default()
        }
    }

    pub(super) fn fail(&mut self, e: HogpError) -> Step {
        let was_ready = self.phase == Phase::Ready;
        (self.phase, self.pending) = (Phase::Failed, None);
        let mut step = Step::default();
        if was_ready {
            step.input = self.release_all();
        }
        step.outcome = Some(Outcome::Failed(e));
        step
    }
}

/// The request a response opcode answers: one less, for every pair used
/// here (Vol 3, Part F, 3.4.8).
pub(super) fn response_to(op: u8) -> u8 {
    op.wrapping_sub(1)
}
