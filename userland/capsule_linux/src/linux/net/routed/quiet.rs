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

//! When an open stream asks net.socks5 what has come. A stream whose last
//! ask brought bytes asks again at once. One whose last ask brought nothing
//! cost the serve loop the slice net.socks5 holds a quiet poll for, so it
//! asks again only after QUIET_GAP_MS, and only if no other quiet stream
//! has asked in the same pass of the serve loop (pass.rs): a pass waits on
//! one quiet stream at most, not on each, unless one has waited
//! QUIET_MAX_MS. Pure, so a host proof holds it
//! (capsule_linux_proofs, routed_quiet_tests).

/// How long a stream whose last ask brought nothing waits to ask again.
pub const QUIET_GAP_MS: i64 = 100;

/// How long it waits at most: past this it asks without the pass's slot,
/// so input arriving pass after pass, a long drag say, never starves it.
pub const QUIET_MAX_MS: i64 = 400;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Quiet {
    /// When the last ask that brought nothing was made; None once bytes came.
    at: Option<i64>,
}

impl Quiet {
    /// Whether to ask at `now`. `slot` takes the pass's one quiet wait, and
    /// is called only for a quiet stream whose gap is over.
    pub fn may_ask(&self, now: i64, slot: impl FnOnce() -> bool) -> bool {
        match self.at {
            None => true,
            Some(at) => {
                let waited = now.saturating_sub(at);
                waited >= QUIET_GAP_MS && (waited >= QUIET_MAX_MS || slot())
            }
        }
    }

    /// What the ask made at `now` brought: `got` bytes.
    pub fn heard(&mut self, now: i64, got: usize) {
        self.at = (got == 0).then_some(now);
    }
}
