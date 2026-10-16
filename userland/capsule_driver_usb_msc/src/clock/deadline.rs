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

//! A point on the clock a wait gives up at. A clock that cannot be read
//! spends every deadline at once: counted as no time passing, as libc's
//! `Deadline` counts it, a wait begun on a good read would never end.

use super::now_ms;

#[derive(Clone, Copy)]
pub struct Deadline {
    end: Option<u64>,
}

impl Deadline {
    pub fn after_ms(ms: u64) -> Self {
        Self { end: now_ms().map(|t| t.saturating_add(ms)) }
    }

    /// What is left of it; nothing once it has passed.
    pub fn left_ms(&self) -> u64 {
        match (self.end, now_ms()) {
            (Some(end), Some(now)) => end.saturating_sub(now),
            _ => 0,
        }
    }

    pub fn expired(&self) -> bool {
        self.left_ms() == 0
    }
}
