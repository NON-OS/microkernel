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

//! What a session keeps to roam: the least secret its AKM needs, the tag
//! naming its passphrase in the PMKSA cache, the security it runs, how well
//! its AP is heard, and when it may next look for a better one.

use super::kit_join::roam_policy;
use super::secret::Secret;
use super::signal::Signal;
use crate::pmksa::Tag;
use crate::rsn::{Ciphers, JoinPolicy, Selection};
use crate::wpa::akm::Akm;

/// After a look, or a move, none starts again for this long: an AP heard
/// weak for a while does not keep the station off its channel.
pub const COOLDOWN_MS: u64 = 30_000;

pub struct Kit {
    pub secret: Secret,
    pub tag: Option<Tag>,
    pub wpa3_only: bool,
    pub policy: JoinPolicy,
    pub runs: Selection,
    /// The group management cipher suite the session's AP named.
    pub group_mgmt: Option<u32>,
    pub signal: Signal,
    pub next_look_ms: u64,
}

impl Kit {
    pub fn new(secret: Secret, runs: Selection, group_mgmt: Option<u32>, ciphers: Ciphers) -> Self {
        let policy = roam_policy(&runs, ciphers);
        let (tag, wpa3_only) = (None, runs.akm == Akm::Sae);
        let (signal, next_look_ms) = (Signal::default(), 0);
        Self { secret, tag, wpa3_only, policy, runs, group_mgmt, signal, next_look_ms }
    }

    pub fn due(&self, now_ms: u64) -> bool {
        self.signal.weak() && now_ms >= self.next_look_ms
    }

    pub fn rest(&mut self, now_ms: u64) {
        self.next_look_ms = now_ms.saturating_add(COOLDOWN_MS);
    }
}
