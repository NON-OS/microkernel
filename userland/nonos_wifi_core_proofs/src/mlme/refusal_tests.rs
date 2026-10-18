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

//! A network saved as WPA3 is refused when it shows PSK only, frames from
//! another BSS or for another station are ignored, and refusals and a
//! deauthentication end the join with their own codes.

use crate::mlme::frames::{assoc_resp, mgmt, open_auth_ok};
use crate::mlme::sim::{beacon, join, rsne, AP, OTHER, STA};
use nonos_wifi_core::mlme::{MlmeFailure, MlmeState};
use nonos_wifi_core::rsn::{JoinPolicy, SelectError};

#[test]
fn a_network_saved_as_wpa3_is_refused_when_it_shows_psk_only() {
    let mut m = join(JoinPolicy::WPA3_ONLY);
    assert!(m.on_mgmt(&beacon(AP, &[&rsne(&[2], 0, 4)])).tx.is_none(), "nothing is sent");
    assert_eq!(m.failure(), Some(MlmeFailure::Select(SelectError::Downgrade)));
}

#[test]
fn frames_from_another_bss_or_for_another_station_are_ignored() {
    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&rsne(&[2], 0, 4)]));
    // A refusal from another AP, and the right AP refusing another station.
    let refuse_other = mgmt(11, OTHER, STA, &[0, 0, 2, 0, 1, 0]);
    let refuse_them = mgmt(11, AP, OTHER, &[0, 0, 2, 0, 1, 0]);
    assert!(m.on_mgmt(&refuse_other).tx.is_none());
    assert!(m.on_mgmt(&refuse_them).tx.is_none());
    assert_eq!(m.state(), MlmeState::Authenticating, "neither ended the join");
    assert!(m.on_mgmt(&open_auth_ok(AP, STA)).tx.is_some());
    let mut other_resp = assoc_resp(17);
    other_resp[10..16].copy_from_slice(&OTHER);
    other_resp[16..22].copy_from_slice(&OTHER);
    m.on_mgmt(&other_resp);
    assert_eq!(m.state(), MlmeState::Associating);
}

#[test]
fn refusals_end_the_join_with_their_codes() {
    let psk_rsne = rsne(&[2], 0, 4);
    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&psk_rsne]));
    m.on_mgmt(&mgmt(11, AP, STA, &[0, 0, 2, 0, 17, 0]));
    assert_eq!(m.failure(), Some(MlmeFailure::AuthRejected(17)));

    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&psk_rsne]));
    m.on_mgmt(&open_auth_ok(AP, STA));
    m.on_mgmt(&assoc_resp(30));
    assert_eq!(m.state(), MlmeState::Associating, "status 30 waits for the retransmit");
    m.on_mgmt(&assoc_resp(18));
    assert_eq!(m.failure(), Some(MlmeFailure::AssocRejected(18)));

    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&psk_rsne]));
    m.on_mgmt(&open_auth_ok(AP, STA));
    m.on_mgmt(&assoc_resp(0));
    m.on_mgmt(&mgmt(12, AP, STA, &[15, 0]));
    assert_eq!(m.failure(), Some(MlmeFailure::Left(15)), "deauthenticated in the four-way");
}
