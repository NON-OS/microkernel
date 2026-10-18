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

//! The kit a join gives its session, and the policy every roam of it runs.

use super::kit::Kit;
use super::secret::Secret;
use crate::pmksa::PmksaCache;
use crate::rsn::{Ciphers, JoinPolicy, Selection};
use crate::softmac::join::{ConnectRequest, Report};
use crate::wpa::akm::Akm;

/// The policy a roam joins under, built from what the session runs and
/// never from the person's wider one: SAE only after SAE, PSK only after
/// PSK, so a transition network's two halves are never mixed in a session.
pub fn roam_policy(runs: &Selection, ciphers: Ciphers) -> JoinPolicy {
    let sae = runs.akm == Akm::Sae;
    JoinPolicy { allow_psk: !sae, allow_sae: sae, ciphers }
}

impl Kit {
    /// The kit for the session a connect request's join opened.
    pub fn for_join(
        req: &ConnectRequest,
        rep: &Report,
        cache: &PmksaCache,
        c: Ciphers,
        now: u64,
    ) -> Option<Self> {
        let runs = rep.selection?;
        let secret = Secret::new(runs.akm, runs.sae_h2e, req.ssid, req.pass)?;
        let mut kit = Self::new(secret, runs, rep.group_mgmt, c);
        (kit.tag, kit.next_look_ms) = (cache.tag(req.ssid, req.pass), now);
        Some(kit)
    }
}
