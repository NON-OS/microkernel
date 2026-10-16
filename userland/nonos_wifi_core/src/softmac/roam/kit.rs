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

//! What a session keeps to roam: the passphrase, the person's policy, the
//! security its join selected, how well its AP is heard, and when it may
//! next look for a better one.

use super::secret::Secret;
use super::signal::Signal;
use crate::rsn::{Ciphers, JoinPolicy, Selection};
use crate::softmac::join::ConnectRequest;
use crate::wpa::akm::Akm;

/// After a look, or a move, none starts again for this long: an AP heard
/// weak for a while does not keep the station off its channel.
pub const COOLDOWN_MS: u64 = 30_000;

pub struct Kit {
    pub secret: Secret,
    pub wpa3_only: bool,
    pub policy: JoinPolicy,
    pub runs: Selection,
    pub signal: Signal,
    pub next_look_ms: u64,
}

impl Kit {
    /// The kit for a session the join selected `runs` for. A roam keeps
    /// the AKM the session runs: SAE only after SAE, PSK only after PSK,
    /// so a transition network's two halves are never mixed in one session.
    pub fn new(
        pass: &[u8],
        wpa3_only: bool,
        runs: Selection,
        ciphers: Ciphers,
        now_ms: u64,
    ) -> Option<Self> {
        let policy =
            if runs.akm == Akm::Sae { JoinPolicy::WPA3_ONLY } else { JoinPolicy::PSK_ONLY };
        let (secret, policy, signal) =
            (Secret::new(pass)?, policy.with_ciphers(ciphers), Signal::default());
        Some(Self { secret, wpa3_only, policy, runs, signal, next_look_ms: now_ms })
    }

    /// The kit for the session a connect request's join opened.
    pub fn for_join(
        req: &ConnectRequest,
        runs: Option<Selection>,
        ciphers: Ciphers,
        now: u64,
    ) -> Option<Self> {
        Self::new(req.pass, req.wpa3_only, runs?, ciphers, now)
    }

    pub fn due(&self, now_ms: u64) -> bool {
        self.signal.weak() && now_ms >= self.next_look_ms
    }

    pub fn rest(&mut self, now_ms: u64) {
        self.next_look_ms = now_ms.saturating_add(COOLDOWN_MS);
    }
}
