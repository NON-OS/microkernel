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

//! Reassociation in the MLME (802.11-2020, 9.3.3.7, 9.3.3.8, 11.3.5.4): a
//! move names the AP left after the listen interval and takes only a
//! Reassociation Response; an ordinary join is unchanged.

use crate::ap_sim::STA_RSNE_PSK;
use crate::mlme::frames::{mgmt, open_auth_ok};
use crate::mlme::sim::{beacon, join, AP, OTHER, STA};
use nonos_wifi_core::mlme::{Mlme, MlmeFailure, MlmeState};
use nonos_wifi_core::rsn::JoinPolicy;

fn answer(subtype: u8, status: u16) -> Vec<u8> {
    let [s0, s1] = status.to_le_bytes();
    mgmt(subtype, AP, STA, &[0x11, 0x04, s0, s1, 0x01, 0xc0])
}

fn requested(from: Option<[u8; 6]>) -> (Mlme, Vec<u8>) {
    let mut m = join(JoinPolicy::PSK_ONLY);
    if let Some(f) = from {
        m.reassociate_from(f);
    }
    m.on_mgmt(&beacon(AP, &[&STA_RSNE_PSK]));
    let req = m.on_mgmt(&open_auth_ok(AP, STA)).tx.expect("a request");
    (m, req)
}

#[test]
fn a_move_sends_a_reassociation_request_naming_the_ap_left() {
    let (_, req) = requested(Some(OTHER));
    assert_eq!((req[0], &req[4..10]), (0x20, &AP[..]), "a Reassociation Request to the new AP");
    let body = &req[24..];
    assert_eq!(body[2..4], [10, 0], "listen interval");
    assert_eq!(body[4..10], OTHER, "Current AP Address");
    assert_eq!(body[10..12], [0, 4], "then the SSID element");
    assert_eq!(&body[12..16], b"home");
}

#[test]
fn a_move_takes_only_a_reassociation_response() {
    let (mut m, _) = requested(Some(OTHER));
    m.on_mgmt(&answer(1, 0));
    assert_eq!(m.state(), MlmeState::Associating, "an association response is not the answer");
    m.on_mgmt(&answer(3, 0));
    assert_eq!(m.state(), MlmeState::FourWay);
    let (mut m, _) = requested(Some(OTHER));
    m.on_mgmt(&answer(3, 17));
    assert_eq!(m.failure(), Some(MlmeFailure::AssocRejected(17)));
}

#[test]
fn an_ordinary_join_is_unchanged() {
    let (mut m, req) = requested(None);
    assert_eq!(req[0], 0x00, "Association Request");
    assert_eq!(req[24 + 4..24 + 6], [0, 4], "the SSID element straight after the listen interval");
    m.on_mgmt(&answer(3, 0));
    assert_eq!(m.state(), MlmeState::Associating, "a reassociation response is not its answer");
    m.on_mgmt(&answer(1, 0));
    assert_eq!(m.state(), MlmeState::FourWay);
}
