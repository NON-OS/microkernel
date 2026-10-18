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

//! The first attempt of a join, on a cached PMKSA when one is held for this
//! AP, network, station address and passphrase (IEEE Std 802.11-2020,
//! 12.6.10.3). An AP that forgot the PMKSA, never caches, or refuses Open
//! System authentication for SAE refuses it; then, or on any other ending
//! short of joined, the entry is forgotten and SAE runs again on fresh
//! randomness. What a joined SAE attempt holds is kept for the next join.

use super::attempt::attempt;
use super::extra::Extra;
use super::joined::JoinError;
use super::joining::Joining;
use super::report::{Outcome, Report};
use super::request::ConnectRequest;
use crate::pmksa::{PmksaCache, Tag};
use crate::regdom::Domain;
use crate::rsn::JoinPolicy;
use crate::softmac::radio::{Env, Radio};

/// Whether `report` calls for the attempt again without the cache.
pub fn retry_with_sae(report: &Report) -> bool {
    report.used_pmksa && !matches!(report.outcome, Outcome::Joined { .. })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn first_attempt<R: Radio, E: Env>(
    r: &mut R,
    env: &mut E,
    domain: Domain,
    req: &ConnectRequest,
    policy: JoinPolicy,
    beacon: &[u8],
    bssid: [u8; 6],
    cache: &mut PmksaCache,
    j: &Joining,
) -> Result<Report, JoinError> {
    let (now, sta) = (env.now_ms(), r.mac());
    let offer = j.tag.and_then(|t| cache.find((bssid, sta), req.ssid, t, now));
    let (from, kept) = (j.from, j.kept);
    let report = attempt(r, env, domain, req, policy, beacon, Extra { offer, from, kept })?;
    if !retry_with_sae(&report) {
        return Ok(report);
    }
    cache.remove(&bssid, req.ssid);
    attempt(r, env, domain, req, policy, beacon, Extra { offer: None, from, kept })
}

/// Keep what a joined SAE attempt holds.
pub(crate) fn keep<E: Env>(
    c: &mut PmksaCache,
    env: &E,
    sta: [u8; 6],
    req: &ConnectRequest,
    t: Option<&Tag>,
    report: &Report,
) {
    if let (Some(p), Outcome::Joined { .. }, Some(t)) = (&report.pmksa, &report.outcome, t) {
        c.keep(sta, req.ssid, t, p.again(), env.now_ms());
    }
}
