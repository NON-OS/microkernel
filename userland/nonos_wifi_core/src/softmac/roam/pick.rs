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

//! Which AP heard in a look the station moves to. Only one of the same
//! ESS counts (IEEE Std 802.11-2020, 11.3.5.4 reassociation is within an
//! ESS): the same SSID, and an RSNE from which the person's policy selects
//! exactly what the session runs, the same AKM, pairwise and group ciphers
//! and management frame protection. A move to a network that would choose
//! otherwise is a different join, not a roam. And it must be heard at
//! least `HYSTERESIS_DB` stronger than the current AP, so two APs heard
//! about the same do not take the station back and forth.

use alloc::vec::Vec;

use crate::dot11::ies::{ap_rates, beacon_tags, find_element, EID_RSN, EID_RSNX};
use crate::dot11::parse::parse_beacon;
use crate::regdom::Band;
use crate::rsn::rsnxe::advertises_h2e;
use crate::rsn::{parse_rsne, select, JoinPolicy, Selection};

pub const HYSTERESIS_DB: i16 = 8;

/// One AP of the network heard in a look.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Heard {
    pub bssid: [u8; 6],
    pub band: Band,
    pub channel: u8,
    pub rssi_dbm: i8,
    pub beacon: Vec<u8>,
}

/// What `policy` would select from the security `beacon` offers.
pub fn offered(beacon: &[u8], policy: JoinPolicy) -> Option<Selection> {
    let tags = beacon_tags(beacon)?;
    let rsne = parse_rsne(&find_element(tags, EID_RSN)?[2..])?;
    let rates = ap_rates(tags);
    let h2e =
        find_element(tags, EID_RSNX).is_some_and(|e| advertises_h2e(&e[2..])) || rates.requires_h2e;
    select(&rsne, h2e, policy).ok()
}

/// The index of the AP to move to, if any: the strongest that qualifies.
pub fn pick(
    current: ([u8; 6], i16),
    ssid: &[u8],
    runs: &Selection,
    policy: JoinPolicy,
    heard: &[Heard],
) -> Option<usize> {
    let same = |s: Selection| {
        (s.akm, s.pairwise, s.group, s.pmf) == (runs.akm, runs.pairwise, runs.group, runs.pmf)
    };
    let qualifies = |h: &Heard| {
        h.bssid != current.0
            && i16::from(h.rssi_dbm) >= current.1.saturating_add(HYSTERESIS_DB)
            && parse_beacon(&h.beacon).is_some_and(|b| b.ssid == ssid && b.bssid == h.bssid)
            && offered(&h.beacon, policy).is_some_and(same)
    };
    (0..heard.len()).filter(|&i| qualifies(&heard[i])).max_by_key(|&i| heard[i].rssi_dbm)
}
