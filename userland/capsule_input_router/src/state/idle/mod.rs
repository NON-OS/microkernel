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

//! Idle: the time since the router last took input from any source, and
//! the screen blank it leads to. The policy's screen timeout (minutes, 0
//! never) blanks the screen once that long passed with no input and no
//! client holding it awake (an inhibit, as zwp_idle_inhibit_manager_v1
//! asks for a video); the next input wakes it. Pure: what the compositor is
//! told, and when, is proven on the host (input_proofs idle_tests).

/// Clients that may hold the screen awake at once.
pub const MAX_INHIBITORS: usize = 16;
/// How soon a refused or unanswered word to the compositor is tried again.
pub const RETRY_MS: i64 = 1000;
const MINUTE_MS: i64 = 60_000;

mod auto_lock;
mod inhibit;
mod tell;

pub use auto_lock::AutoLock;

pub struct Idle {
    /// Uptime of the last input, from any source, routed or not.
    pub last_input_ms: i64,
    /// When the screen was blanked; None while it shows.
    pub blanked_at: Option<i64>,
    /// The screen timeout in ms; 0 never blanks.
    pub blank_ms: i64,
    inhibitors: [u32; MAX_INHIBITORS],
    /// What the compositor last took, and the last word tried and when.
    pub told: Option<bool>,
    pub tried: Option<(bool, i64)>,
}

impl Idle {
    pub const fn new(now_ms: i64) -> Idle {
        let inhibitors = [0; MAX_INHIBITORS];
        let (told, tried) = (None, None);
        Idle { last_input_ms: now_ms, blanked_at: None, blank_ms: 0, inhibitors, told, tried }
    }

    pub fn input(&mut self, now_ms: i64) {
        self.last_input_ms = self.last_input_ms.max(now_ms);
    }

    /// The policy's screen timeout, in minutes.
    pub fn set_timeout_minutes(&mut self, minutes: u8) {
        self.blank_ms = i64::from(minutes) * MINUTE_MS;
    }

    /// Whether the screen is to be blank at `now_ms`. Only input wakes a
    /// blank screen; an inhibit taken after the blank does not.
    pub fn step(&mut self, now_ms: i64) -> bool {
        match self.blanked_at {
            Some(at) if self.last_input_ms > at || self.blank_ms == 0 => self.blanked_at = None,
            None if self.blank_ms > 0 && !self.inhibited() => {
                if now_ms.saturating_sub(self.last_input_ms) >= self.blank_ms {
                    self.blanked_at = Some(now_ms);
                }
            }
            _ => {}
        }
        self.blanked_at.is_some()
    }
}
