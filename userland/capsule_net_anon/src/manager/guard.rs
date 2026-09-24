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

//! The one guard this session keeps, and when it gives up on it.

use crate::path::{choose, Position, Relay};
use crate::trace;

use super::roll::roll;
use super::state::Manager;

/// Failed connections before the guard is replaced.
const GUARD_ATTEMPTS: u8 = 3;

const GUARD_RETRY_SECONDS: u64 = 5;

/// The guard this session uses, with what is known about reaching it.
#[derive(Clone)]
pub struct Guard {
    pub relay: Relay,
    pub failures: u8,
    /// When the last attempt was made, successful or not.
    pub tried_at: u64,
}

impl Guard {
    pub fn new(relay: Relay) -> Self {
        Self { relay, failures: 0, tried_at: 0 }
    }

    /// Whether this guard should be dialled again now.
    pub fn due(&self, now: u64) -> bool {
        now >= self.tried_at.saturating_add(GUARD_RETRY_SECONDS)
    }

    /// Whether it has failed often enough to be replaced.
    pub fn exhausted(&self) -> bool {
        self.failures >= GUARD_ATTEMPTS
    }
}

/// Draw a guard from the consensus. `None` when the pool offers none.
pub fn draw(state: &Manager) -> Option<Guard> {
    let roll = roll()?;
    match choose(&state.relays, &state.weights, Position::Guard, &[], roll) {
        Some(relay) => Some(Guard::new(relay.clone())),
        None => {
            trace::say(b"no guard in the consensus can be used");
            None
        }
    }
}
