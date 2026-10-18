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

//! One beacon or probe response judged for a join: whether it names the
//! network, and if so whether this station could join it as it claims to be,
//! with the AKM and protection it would run. The join and the candidate list
//! both judge by this one rule.

use super::failure::MlmeFailure;
use crate::dot11::ies::{ap_rates, beacon_tags, find_element, Rates, EID_RSN, EID_RSNX};
use crate::dot11::parse::{parse_beacon, BeaconInfo};
use crate::rsn::rsnxe::advertises_h2e;
use crate::rsn::{parse_rsne, select, JoinPolicy, Selection};

/// A frame this station could join from: its RSNE and RSNXE elements whole.
pub struct Judged<'a> {
    pub info: BeaconInfo<'a>,
    pub selection: Selection,
    pub rsne: &'a [u8],
    pub rsnxe: Option<&'a [u8]>,
    pub rates: Rates,
}

/// `None` when `frame` is no beacon or probe response for `ssid`; otherwise
/// what joining from it would run, or why it cannot be joined.
pub fn judge<'a>(
    frame: &'a [u8],
    ssid: &[u8],
    policy: JoinPolicy,
) -> Option<Result<Judged<'a>, MlmeFailure>> {
    let info = parse_beacon(frame)?;
    if info.ssid != ssid {
        return None;
    }
    let tags = beacon_tags(frame)?;
    let Some(rsne) = find_element(tags, EID_RSN) else {
        return Some(Err(MlmeFailure::OpenNetwork));
    };
    let Some(parsed) = parse_rsne(&rsne[2..]) else {
        return Some(Err(MlmeFailure::MalformedRsne));
    };
    let rsnxe = find_element(tags, EID_RSNX);
    let rates = ap_rates(tags);
    if rates.requires_ht {
        return Some(Err(MlmeFailure::NeedsHt));
    }
    let h2e = rsnxe.is_some_and(|e| advertises_h2e(&e[2..])) || rates.requires_h2e;
    Some(match select(&parsed, h2e, policy) {
        Ok(selection) => Ok(Judged { info, selection, rsne, rsnxe, rates }),
        Err(e) => Err(MlmeFailure::Select(e)),
    })
}
