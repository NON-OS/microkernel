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

//! How long before every port decided against is tried again: a minute at
//! first, for a phone whose switch to tethering the connect-change bit did
//! not show, then twice as long after each round that bound nothing, up to
//! half an hour. A device that fails every bind costs the thread that
//! answers net.core a share of its time that shrinks, not one bind a minute
//! for ever. The wait itself is pure, for the proofs.

use nonos_libc::Deadline;

const FIRST_MS: u64 = 60_000;
const MOST_MS: u64 = 30 * 60_000;

/// The wait after `rounds` retries in a row bound nothing.
pub fn retry_after_ms(rounds: u32) -> u64 {
    FIRST_MS.saturating_mul(1 << rounds.min(16)).min(MOST_MS)
}

/// When every port decided against is next tried again.
pub(super) struct Backoff {
    at: Deadline,
    /// Retries of every port in a row that bound nothing.
    rounds: u32,
}

impl Backoff {
    pub(super) fn new() -> Self {
        Self { at: Deadline::after_ms(retry_after_ms(0)), rounds: 0 }
    }

    /// Whether the retry is due now; each one puts the next further off.
    pub(super) fn due(&mut self) -> bool {
        if !self.at.expired() {
            return false;
        }
        self.rounds = self.rounds.saturating_add(1);
        self.at = Deadline::after_ms(retry_after_ms(self.rounds));
        true
    }

    /// A device was bound: the next retry is the first wait away again.
    pub(super) fn reset(&mut self) {
        *self = Self::new();
    }
}
