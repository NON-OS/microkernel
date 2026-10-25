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

//! The running totals of a multi-packet ping.

#[path = "emit_summary.rs"]
mod emit_summary;

pub use emit_summary::emit_summary;

// Running totals for a multi-packet ping, so the summary can report loss and
// round-trip spread the way people expect from ping.
pub struct Stats {
    sent: u32,
    recv: u32,
    min: u64,
    max: u64,
    sum: u64,
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

impl Stats {
    pub fn new() -> Self {
        Self { sent: 0, recv: 0, min: u64::MAX, max: 0, sum: 0 }
    }

    pub fn record(&mut self, rtt: u64) {
        self.sent += 1;
        self.recv += 1;
        self.sum += rtt;
        self.min = self.min.min(rtt);
        self.max = self.max.max(rtt);
    }

    pub fn miss(&mut self) {
        self.sent += 1;
    }
}
