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

//! What the bound interface's link did since it was last asked.
//!
//! The stack is bound to the first port whose link answers up, and nothing
//! looked at that link again. A cable pulled and put back, or a Wi-Fi network
//! left and joined again, kept an address that may belong to another network
//! until its lease ran out, hours later.

/// How often the bound link is asked.
pub const WATCH_EVERY_MS: i64 = 1_000;

/// The link went down, or is up again after going down.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Change {
    Dropped,
    Returned,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct LinkWatch {
    port: u32,
    down: bool,
    next_ms: i64,
}

impl LinkWatch {
    pub const fn new() -> Self {
        Self { port: 0, down: false, next_ms: 0 }
    }

    pub fn due(&self, now_ms: i64) -> bool {
        now_ms >= self.next_ms
    }

    /// What the link of `port` answering `up` means. `None` is a driver
    /// that did not answer in time, which says nothing about the link. A
    /// port not watched before starts as up: it was bound because it was.
    pub fn observe(&mut self, now_ms: i64, port: u32, up: Option<bool>) -> Option<Change> {
        self.next_ms = now_ms + WATCH_EVERY_MS;
        if port != self.port {
            self.port = port;
            self.down = false;
        }
        match up {
            Some(false) if !self.down => {
                self.down = true;
                Some(Change::Dropped)
            }
            Some(true) if self.down => {
                self.down = false;
                Some(Change::Returned)
            }
            _ => None,
        }
    }
}
