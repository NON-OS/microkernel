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
//! ESS): the same SSID, an RSNE from which the session's own policy
//! selects exactly the `Selection` it runs (AKM, pairwise and group
//! ciphers, management frame protection, and SAE's hash-to-element), and
//! the same group management cipher. A move to a network that would choose
//! otherwise is a different join, not a roam. A candidate a roam already
//! failed to is barred for the boot. And it must be heard at least
//! `HYSTERESIS_DB` stronger than the current AP, so two APs heard about
//! the same do not take the station back and forth.

use super::offer::{group_mgmt_of, offered, Heard};
use crate::dot11::parse::parse_beacon;
use crate::rsn::{JoinPolicy, Selection};

pub const HYSTERESIS_DB: i16 = 8;

/// What a candidate must match: the session's network and security, and
/// the APs already barred.
pub struct Want<'a> {
    pub ssid: &'a [u8],
    pub runs: &'a Selection,
    pub group_mgmt: Option<u32>,
    pub policy: JoinPolicy,
    pub barred: &'a [[u8; 6]],
}

/// The index of the AP to move to, if any: the strongest that qualifies.
pub fn pick(current: ([u8; 6], i16), w: &Want, heard: &[Heard]) -> Option<usize> {
    let qualifies = |h: &Heard| {
        h.bssid != current.0
            && !w.barred.contains(&h.bssid)
            && i16::from(h.rssi_dbm) >= current.1.saturating_add(HYSTERESIS_DB)
            && parse_beacon(&h.beacon).is_some_and(|b| b.ssid == w.ssid && b.bssid == h.bssid)
            && offered(&h.beacon, w.policy).as_ref() == Some(w.runs)
            && group_mgmt_of(&h.beacon) == w.group_mgmt
    };
    (0..heard.len()).filter(|&i| qualifies(&heard[i])).max_by_key(|&i| heard[i].rssi_dbm)
}
