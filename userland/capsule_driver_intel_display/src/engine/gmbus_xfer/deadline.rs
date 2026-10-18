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

//! One GMBUS transaction's deadline. Every wait in it draws on this one
//! end rather than a fresh limit of its own, so a sink that stretches each
//! byte just short of a step's limit still cannot hold the bus past it;
//! and a caller with a budget of its own (a whole EDID read) hands each
//! transaction the sooner of the two ends.

use super::super::access::{Clock, Expiry};

/// The longest one transaction may take. An honest 128-byte block at
/// DDC's 100 kHz is about 12 ms of bus time (130 bytes of nine clocks;
/// recalled, unverified); the rest is room for a sink holding the clock.
pub const XFER_US: u64 = 100_000;

pub struct Deadline<'a> {
    clock: &'a dyn Clock,
    end: Option<u64>,
}

impl<'a> Deadline<'a> {
    pub fn new(clock: &'a dyn Clock, us: u64) -> Self {
        Deadline { clock, end: clock.now_us().map(|t| t.saturating_add(us)) }
    }

    /// `us` from now, or this deadline's end when that comes first.
    pub fn within(&self, us: u64) -> Deadline<'a> {
        let next = Deadline::new(self.clock, us);
        Deadline { clock: self.clock, end: self.end.zip(next.end).map(|(a, b)| a.min(b)) }
    }
}

impl Expiry for Deadline<'_> {
    fn expired(&self) -> bool {
        match (self.end, self.clock.now_us()) {
            (Some(end), Some(now)) => now >= end,
            _ => true,
        }
    }
}
