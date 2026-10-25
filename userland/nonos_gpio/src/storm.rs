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

//! A pin that interrupts faster than any device has reason to, counted
//! over a fixed window: past `LIMIT` events in `WINDOW_MS` it is a storm,
//! and the router masks that pin alone. A stuck line or a floating input
//! then costs one pin, not the controller's shared interrupt.

pub const WINDOW_MS: u64 = 1000;
pub const LIMIT: u32 = 500;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Storm {
    since_ms: u64,
    count: u32,
}

impl Storm {
    /// Count one event at `now_ms`; true when it makes a storm.
    pub fn event(&mut self, now_ms: u64) -> bool {
        if now_ms.saturating_sub(self.since_ms) >= WINDOW_MS {
            *self = Storm { since_ms: now_ms, count: 0 };
        }
        self.count += 1;
        self.count > LIMIT
    }
}
