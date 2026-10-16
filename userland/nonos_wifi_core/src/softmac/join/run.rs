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

//! One whole join: hunt the network, tune and filter to it, and run the
//! state machine under the person's policy. A network offering WPA3 and
//! WPA2 is joined with WPA3 first, then, unless the password was wrong or
//! the network is saved as WPA3, with the WPA2 it offers every station.

use super::attempt::{attempt, retry_with_psk};
use super::cached::{first_attempt, keep};
use super::extra::Extra;
use super::hunt::hunt;
use super::joining::Joining;
use super::report::{Outcome, Report};
use super::request::ConnectRequest;
use crate::pmksa::PmksaCache;
use crate::regdom::{Band, Domain};
use crate::rsn::JoinPolicy;
use crate::softmac::radio::{Env, Radio};

pub enum JoinError {
    /// Not heard; how many beacons of other networks were.
    NotFound(u32),
    NoEntropy,
}

/// A finished attempt and where the network was heard.
pub struct Joined {
    pub report: Report,
    pub band: Band,
    pub channel: u8,
}

pub fn join<R: Radio, E: Env>(
    r: &mut R,
    env: &mut E,
    domain: Domain,
    req: &ConnectRequest,
    hints: &[(Band, u8)],
    cache: &mut PmksaCache,
) -> Result<Joined, JoinError> {
    let (found, beacons) = hunt(r, env, domain, req.ssid, req.hidden, hints);
    let found = found.ok_or(JoinError::NotFound(beacons))?;
    r.tune(found.band, found.channel);
    let bssid: [u8; 6] = found.beacon[16..22].try_into().unwrap_or([0; 6]);
    r.set_bss(Some(bssid));
    let ciphers = r.ciphers();
    let first = if req.wpa3_only { JoinPolicy::WPA3_ONLY } else { JoinPolicy::ANY };
    let (policy, psk) = (first.with_ciphers(ciphers), JoinPolicy::PSK_ONLY.with_ciphers(ciphers));
    let (tag, beacon) = (cache.tag(req.ssid, req.pass), &found.beacon);
    let j = Joining { tag: tag.as_ref(), ..Joining::default() };
    let mut report = first_attempt(r, env, domain, req, policy, beacon, bssid, cache, &j)?;
    if retry_with_psk(&report, req.wpa3_only) {
        report = attempt(r, env, domain, req, psk, beacon, Extra::default())?;
    }
    keep(cache, env, r.mac(), req, tag.as_ref(), &report);
    if !matches!(report.outcome, Outcome::Joined { .. }) {
        r.associated(None);
        r.set_bss(None);
    }
    Ok(Joined { report, band: found.band, channel: found.channel })
}
