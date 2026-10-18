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

//! What an AP heard in a look offers: its beacon, and the security the
//! session's policy would select from it.

use alloc::vec::Vec;

use crate::dot11::ies::{ap_rates, beacon_tags, find_element, EID_RSN, EID_RSNX};
use crate::regdom::Band;
use crate::rsn::rsnxe::advertises_h2e;
use crate::rsn::{parse_rsne, select, JoinPolicy, Rsne, Selection};

/// One AP of the network heard in a look.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Heard {
    pub bssid: [u8; 6],
    pub band: Band,
    pub channel: u8,
    pub rssi_dbm: i8,
    pub beacon: Vec<u8>,
}

fn rsne_of(beacon: &[u8]) -> Option<Rsne> {
    parse_rsne(&find_element(beacon_tags(beacon)?, EID_RSN)?[2..])
}

/// What `policy` would select from the security `beacon` offers.
pub fn offered(beacon: &[u8], policy: JoinPolicy) -> Option<Selection> {
    let tags = beacon_tags(beacon)?;
    let rates = ap_rates(tags);
    let h2e =
        find_element(tags, EID_RSNX).is_some_and(|e| advertises_h2e(&e[2..])) || rates.requires_h2e;
    select(&rsne_of(beacon)?, h2e, policy).ok()
}

/// The group management cipher suite `beacon`'s RSNE names.
pub fn group_mgmt_of(beacon: &[u8]) -> Option<u32> {
    rsne_of(beacon)?.group_mgmt
}
