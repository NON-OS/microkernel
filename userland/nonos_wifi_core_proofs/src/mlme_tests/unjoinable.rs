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

//! Networks this station cannot join, and the rates it offers.

use nonos_wifi_core::mlme::{JoinRequest, Mlme, MlmeFailure};
use nonos_wifi_core::rsn::{JoinPolicy, SelectError};

use super::assoc::assoc_elements;
use super::base::{entropy, join, AP, SSID, STA};
use super::frames::{beacon, open_auth_ok, rsne};

#[test]
fn unjoinable_networks_say_why() {
    let cases: [(Vec<Vec<u8>>, MlmeFailure); 4] = [
        (vec![], MlmeFailure::OpenNetwork),
        (vec![rsne(&[2], 0, 2)], MlmeFailure::Select(SelectError::UnsupportedCipher)),
        (vec![vec![48, 3, 1, 0, 0]], MlmeFailure::MalformedRsne),
        (vec![rsne(&[2], 0, 4), vec![1, 2, 0x82, 0xFF]], MlmeFailure::NeedsHt),
    ];
    for (els, want) in cases {
        let refs: Vec<&[u8]> = els.iter().map(|e| e.as_slice()).collect();
        let mut m = join(JoinPolicy::ANY);
        assert!(m.on_mgmt(&beacon(AP, &refs)).tx.is_none());
        assert_eq!(m.failure(), Some(want));
    }
    let mut m = Mlme::join(&JoinRequest {
        our_mac: STA,
        ssid: SSID,
        passphrase: b"short",
        policy: JoinPolicy::ANY,
        entropy: entropy(),
    });
    m.on_mgmt(&beacon(AP, &[&rsne(&[2], 0, 4)]));
    assert_eq!(m.failure(), Some(MlmeFailure::BadPassphrase));
}

#[test]
fn the_association_request_offers_the_aps_own_rates() {
    // A g-only AP: OFDM rates only, 6, 12 and 24 basic.
    let sr = [1u8, 8, 0x8c, 0x12, 0x98, 0x24, 0xb0, 0x48, 0x60, 0x6c];
    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&sr, &rsne(&[2], 0, 4)]));
    let req = m.on_mgmt(&open_auth_ok(AP, STA)).tx.unwrap();
    let els = assoc_elements(&req);
    assert_eq!(els[1], (1, sr[2..].to_vec()), "its eight rates, basic bits and all");
    assert!(!els.iter().any(|e| e.0 == 50), "no extended rates when eight suffice");
}
