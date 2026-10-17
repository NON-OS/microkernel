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
//! The primary's owner as the watch sees it, a twin of the wire
//! server's for this driver's own protocol: the last pid to ask for the
//! primary (only the compositor may send here, so a new one is a restarted
//! compositor), when it was last heard, whether its death or silence has
//! had the screen blanked whole, and whether its next HEALTHCHECK says LOST.

use core::cell::Cell;

/// Six of the compositor's 500 ms HEALTHCHECK polls missed.
pub const OWNER_SILENCE_MS: u64 = 3_000;

#[derive(Default)]
pub struct Owner {
    pub pid: Cell<u32>,
    pub heard_ms: Cell<u64>,
    /// The blank for this death or silence landed whole.
    pub quiet: Cell<bool>,
    pub lost: Cell<bool>,
}

impl Owner {
    pub const fn new() -> Self {
        Owner {
            pid: Cell::new(0),
            heard_ms: Cell::new(0),
            quiet: Cell::new(false),
            lost: Cell::new(false),
        }
    }

    /// A request from `sender` at `now`; only the owner's count.
    pub fn heard(&self, sender: u32, now: u64) {
        if sender != 0 && sender == self.pid.get() {
            self.heard_ms.set(now);
            self.quiet.set(false);
        }
    }

    /// Whether the screen is to be blanked now: an owner dead, or silent
    /// for OWNER_SILENCE_MS, whose blank has not yet landed whole.
    pub fn due(&self, now: u64, alive: bool) -> bool {
        let silent = now.saturating_sub(self.heard_ms.get()) >= OWNER_SILENCE_MS;
        self.pid.get() != 0 && !self.quiet.get() && (silent || !alive)
    }

    /// LOST for the owner, once for each blank.
    pub fn take_lost(&self, sender: u32) -> bool {
        sender == self.pid.get() && self.lost.replace(false)
    }

    /// `pid` owns the primary from `now`, with nothing of the last owner's.
    pub fn adopt(&self, pid: u32, now: u64) {
        self.pid.set(pid);
        self.heard_ms.set(now);
        self.quiet.set(false);
        self.lost.set(false);
    }
}
