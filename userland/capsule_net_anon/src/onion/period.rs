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


//! Time periods, and which shared random value a fetch uses.
//!
//! All of it is computed from the consensus, as a Tor client computes it, so
//! the client and every HSDir agree whatever this machine's clock says:
//! hs_get_time_period_num, sr_state_get_start_time_of_current_protocol_run
//! and hs_in_period_between_tp_and_srv in the fork.

/// hsdir_interval when the consensus does not set it, in minutes.
pub const PERIOD_MINUTES: u64 = 1440;
/// SHARED_RANDOM_N_ROUNDS: a shared random phase is twelve voting rounds.
const ROUNDS_PER_PHASE: u64 = 12;
/// SHARED_RANDOM_N_PHASES: a protocol run is a commit and a reveal phase.
const PHASES: u64 = 2;

/// The consensus times a lookup is computed from.
#[derive(Clone, Copy, Debug)]
pub struct Clock {
    /// `valid-after`, in seconds since the epoch.
    pub valid_after: u64,
    /// `fresh-until` minus `valid-after`: the voting interval.
    pub interval: u64,
}

impl Clock {
    /// The time period number at `valid_after`. The periods are offset by
    /// one shared random phase, so a new period starts when a new shared
    /// random value has just been published. `None` on a nonsense clock.
    pub fn period(&self) -> Option<u64> {
        let offset_minutes = ROUNDS_PER_PHASE.checked_mul(self.interval)? / 60;
        let minutes = (self.valid_after / 60).checked_sub(offset_minutes)?;
        Some(minutes / PERIOD_MINUTES)
    }

    /// Whether a fetch uses the current shared random value rather than the
    /// previous one: true between the start of a time period and the next
    /// shared random value (the client's fetch_srv in node_set_hsdir_index).
    pub fn current_srv(&self) -> Option<bool> {
        if self.interval == 0 {
            return None;
        }
        let slot = (self.valid_after / self.interval) % (ROUNDS_PER_PHASE * PHASES);
        let run_start = self.valid_after - slot * self.interval;
        let next_period = Clock { valid_after: run_start, interval: self.interval }.period()? + 1;
        let offset_seconds = ROUNDS_PER_PHASE * self.interval;
        let period_start = next_period.checked_mul(PERIOD_MINUTES * 60)?.checked_add(offset_seconds)?;
        let between_srv_and_tp = self.valid_after >= run_start && self.valid_after < period_start;
        Some(!between_srv_and_tp)
    }
}
