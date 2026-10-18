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

//! Where one try of a hub port stands, and the step each look at the port
//! takes it by. Every wait is a deadline on the clock.

use super::error::HubError;
use super::port_status::HubPort;
use super::step::Step;
use super::timing::{DEBOUNCE_MS, RECOVERY_MS, RESET_TIMEOUT_MS, SHORT_RESET_MS};

/// Debounce and Recover hold their deadlines, Reset when it was asked for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Stage {
    #[default]
    Idle,
    Debounce(u64),
    Reset(u64),
    Recover(u64),
}

impl Stage {
    /// The device left: a try ends, and says so once the device was reset.
    pub fn gone(&mut self) -> Step {
        let was = core::mem::replace(self, Stage::Idle);
        match was {
            Stage::Reset(_) | Stage::Recover(_) => Step::Failed(HubError::Gone),
            _ => Step::Wait,
        }
    }

    /// One step of a connected port at `now`.
    pub fn step(&mut self, st: HubPort, now: u64) -> Step {
        let (next, step) = match *self {
            Stage::Idle => (Stage::Debounce(now + DEBOUNCE_MS), Step::Wait),
            // A connection that changed while settling starts it over.
            Stage::Debounce(_) if st.connect_changed => {
                (Stage::Debounce(now + DEBOUNCE_MS), Step::Wait)
            }
            Stage::Debounce(until) if now < until => (*self, Step::Wait),
            Stage::Debounce(_) => (Stage::Reset(now), Step::Reset),
            Stage::Reset(since) if now < since + SHORT_RESET_MS => (*self, Step::Wait),
            Stage::Reset(_) if !st.resetting && st.enabled => {
                (Stage::Recover(now + RECOVERY_MS), Step::ResetOver(None))
            }
            Stage::Reset(_) if !st.resetting && st.reset_changed => {
                (Stage::Idle, Step::ResetOver(Some(HubError::NotEnabled)))
            }
            Stage::Reset(since) if now >= since + RESET_TIMEOUT_MS => {
                (Stage::Idle, Step::Failed(HubError::ResetTimeout))
            }
            Stage::Reset(_) => (*self, Step::Wait),
            Stage::Recover(until) if now < until => (*self, Step::Wait),
            Stage::Recover(_) => (Stage::Idle, Step::Address(st.speed)),
        };
        *self = next;
        step
    }
}
