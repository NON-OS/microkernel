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

//! The MLME's request: the element when asked for and protected, nothing
//! otherwise, which is every caller's request as it was before.

use crate::ap_sim::STA_RSNE_PSK;
use crate::mlme::frames::{assoc_elements, open_auth_ok};
use crate::mlme::sim::{beacon, join, AP, STA};
use crate::pmf_sim::AP_RSNE_MFP;
use nonos_wifi_core::rsn::JoinPolicy;

/// The element: eleven octets, only bit 84 (octet 10, bit 4) set.
pub const BIT_84: [u8; 11] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10];

/// The (re)association request's elements, for `rsne` and with the offer
/// made or not, moving from `from` if given.
pub fn request(rsne: &[u8], offer: bool, from: Option<[u8; 6]>) -> Vec<(u8, Vec<u8>)> {
    let mut m = join(JoinPolicy::ANY);
    if offer {
        m.verify_beacons();
    }
    if let Some(f) = from {
        m.reassociate_from(f);
    }
    m.on_mgmt(&beacon(AP, &[rsne]));
    let req = m.on_mgmt(&open_auth_ok(AP, STA)).tx.expect("a request");
    let skip = if from.is_some() { 6 } else { 0 };
    assoc_elements(&[&req[..28], &req[28 + skip..]].concat())
}

#[test]
fn a_protected_request_from_a_beacon_checker_carries_bit_84_after_the_rsne() {
    let e = request(&AP_RSNE_MFP, true, None);
    let at = e.iter().position(|x| x.0 == 127).expect("Extended Capabilities");
    assert_eq!(e[at].1, BIT_84);
    assert_eq!(e[at - 1].0, 48, "right after the RSNE");
}

#[test]
fn a_reassociation_carries_it_too() {
    let e = request(&AP_RSNE_MFP, true, Some([2, 0, 0, 0, 0, 9]));
    assert!(e.iter().any(|x| x.0 == 127 && x.1 == BIT_84));
}

#[test]
fn without_protection_or_without_the_offer_there_is_no_element() {
    assert!(request(&STA_RSNE_PSK, true, None).iter().all(|x| x.0 != 127), "no PMF");
    assert!(request(&AP_RSNE_MFP, false, None).iter().all(|x| x.0 != 127), "not asked");
}
