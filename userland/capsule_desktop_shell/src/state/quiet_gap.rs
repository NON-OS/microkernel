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

//! How often the shell's one-second tick may ask a service that went quiet.
//!
//! The tick runs on the shell's only thread, between the input it serves. A
//! service that is registered but not answering (the policy store while it
//! reads the disk, the installer while it loads a package) cost its whole
//! reply timeout on every tick, several reads a tick, and the desktop
//! stuttered while it lasted. After a read with no answer the service is let
//! be for a gap that doubles to a ceiling; the first answer resets it.
//!
//! Times are passed in (uptime milliseconds), so the rule is a host test.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QuietGap {
    next_ms: u64,
    gap_ms: u64,
}

impl QuietGap {
    /// The first gap after a read with no answer.
    pub const FIRST_MS: u64 = 1_000;
    /// The longest a quiet service is left before it is asked again.
    pub const MAX_MS: u64 = 16_000;

    pub const fn new() -> Self {
        QuietGap { next_ms: 0, gap_ms: Self::FIRST_MS }
    }

    /// Whether the service may be asked at `now_ms`.
    pub fn due(&self, now_ms: u64) -> bool {
        now_ms >= self.next_ms
    }

    /// A read at `now_ms` got no answer: wait the gap, then double it.
    pub fn missed(&mut self, now_ms: u64) {
        self.next_ms = now_ms.saturating_add(self.gap_ms);
        self.gap_ms = self.gap_ms.saturating_mul(2).min(Self::MAX_MS);
    }

    /// The service answered: ask it on every tick again.
    pub fn answered(&mut self) {
        *self = QuietGap::new();
    }
}

impl Default for QuietGap {
    fn default() -> Self {
        QuietGap::new()
    }
}
