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

//! Keystroke to screen: how long each key took, from the event reaching the
//! terminal to the frame it changed being painted, kept for the last 64 keys
//! so `latency` can say the middle, the slow end and the worst.

/// How many keys are kept.
pub const KEEP: usize = 64;

pub struct Latency {
    ms: [u32; KEEP],
    n: usize,
    next: usize,
}

/// The middle, the 95th percentile and the worst, in milliseconds, and how
/// many keys they come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spread {
    pub p50: u32,
    pub p95: u32,
    pub max: u32,
    pub keys: usize,
}

impl Latency {
    pub const fn new() -> Self {
        Latency { ms: [0; KEEP], n: 0, next: 0 }
    }

    /// Keep one key's time; past `KEEP` the oldest gives way.
    pub fn push(&mut self, ms: u32) {
        self.ms[self.next] = ms;
        self.next = (self.next + 1) % KEEP;
        self.n = (self.n + 1).min(KEEP);
    }

    /// The spread of the kept times, or None before any key.
    pub fn spread(&self) -> Option<Spread> {
        if self.n == 0 {
            return None;
        }
        let mut v = self.ms;
        let kept = &mut v[..self.n];
        kept.sort_unstable();
        let at = |q: usize| kept[(self.n - 1) * q / 100];
        Some(Spread { p50: at(50), p95: at(95), max: kept[self.n - 1], keys: self.n })
    }
}

impl Default for Latency {
    fn default() -> Self {
        Self::new()
    }
}
