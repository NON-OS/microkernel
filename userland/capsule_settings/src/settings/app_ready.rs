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

//! Settings and its readying: the policy store found, then its values read in
//! (hydrated), a pass it did not answer run again after a pause rather than
//! on every event.

use nonos_libc::mk_uptime_ms;

use super::ipc::hydrate_pass::HYDRATE_RETRY_MS;
use super::ipc::{hydrate, lookup_policy_port};
use super::state::{state_new, State};

impl Settings {
    pub(super) fn ensure_ready(&mut self) {
        if !self.state.policy_ready {
            if let Ok(port) = lookup_policy_port() {
                self.state.policy_port = port;
                self.state.policy_ready = true;
            }
        }
        if self.state.policy_ready && !self.hydrated && self.hydrate_due() {
            self.hydrated = hydrate(&mut self.state);
            self.hydrate_due_ms = mk_uptime_ms().saturating_add(HYDRATE_RETRY_MS);
        }
    }

    pub(super) fn hydrate_due(&self) -> bool {
        mk_uptime_ms() >= self.hydrate_due_ms
    }
}

pub struct Settings {
    pub(super) state: State,
    pub(super) hydrated: bool,
    /// When a pass the policy store did not answer is run again, on the
    /// uptime clock.
    pub(super) hydrate_due_ms: i64,
}

impl Settings {
    pub fn new() -> Self {
        let mut state = state_new();
        if let Ok(port) = lookup_policy_port() {
            state.policy_port = port;
            state.policy_ready = true;
        }
        Settings { state, hydrated: false, hydrate_due_ms: 0 }
    }
}
