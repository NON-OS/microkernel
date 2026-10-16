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

//! The first attempt of a join, on a cached PMKSA when one is held for
//! this AP, network and passphrase (IEEE Std 802.11-2020, 12.6.10.3). An
//! AP that no longer holds the PMKSA refuses the association. So does one
//! that never caches, or that refuses Open System authentication for SAE.
//! For any of those, or any other ending short of joined, the entry is
//! forgotten and the attempt runs again with SAE, on fresh randomness.
//! What a joined SAE attempt holds is kept for the next join.

use super::attempt::attempt;
use super::report::{Outcome, Report};
use super::request::ConnectRequest;
use super::run::JoinError;
use crate::pmksa::PmksaCache;
use crate::regdom::Domain;
use crate::rsn::JoinPolicy;
use crate::softmac::radio::{Env, Radio};

/// Whether `report` calls for the attempt again without the cache.
pub fn retry_with_sae(report: &Report) -> bool {
    report.used_pmksa && !matches!(report.outcome, Outcome::Joined { .. })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn first_attempt<R: Radio, E: Env>(
    r: &mut R,
    env: &mut E,
    domain: Domain,
    req: &ConnectRequest,
    policy: JoinPolicy,
    beacon: &[u8],
    bssid: [u8; 6],
    cache: &mut PmksaCache,
) -> Result<Report, JoinError> {
    let offer = cache.find(&bssid, req.ssid, req.pass, env.now_ms());
    let report = attempt(r, env, domain, req, policy, beacon, offer)?;
    if !retry_with_sae(&report) {
        return Ok(report);
    }
    cache.remove(&bssid, req.ssid);
    attempt(r, env, domain, req, policy, beacon, None)
}

/// Keep what a joined SAE attempt holds.
pub(super) fn keep<E: Env>(cache: &mut PmksaCache, env: &E, req: &ConnectRequest, report: &Report) {
    if let (Some(p), Outcome::Joined { .. }) = (report.pmksa, &report.outcome) {
        cache.keep(req.ssid, req.pass, p, env.now_ms());
    }
}
