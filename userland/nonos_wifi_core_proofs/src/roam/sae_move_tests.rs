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

//! A WPA3 move on a cached PMKSA: Open System authentication, then a
//! Reassociation Request naming the AP left whose RSNE lists the PMKID,
//! then the four-way handshake under the cached PMK.

use crate::ap_sim::AP_RSNE_MIXED;
use crate::mlme::frames::{mgmt, open_auth_ok};
use crate::mlme::sim::{beacon, join, AP, OTHER, STA};
use crate::pmksa::sim::{by_sae, four_way};
use nonos_wifi_core::rsn::JoinPolicy;

#[test]
fn a_wpa3_move_reassociates_on_the_new_aps_cached_pmksa() {
    let (first, _) = by_sae();
    let p = first.pmksa().unwrap();
    let mut m = join(JoinPolicy::ANY);
    m.offer_pmksa(p);
    m.reassociate_from(OTHER);
    let auth = m.on_mgmt(&beacon(AP, &[&AP_RSNE_MIXED])).tx.unwrap();
    assert_eq!(auth[24..26], [0, 0], "Open System: SAE is skipped");
    let req = m.on_mgmt(&open_auth_ok(AP, STA)).tx.unwrap();
    assert_eq!((req[0], &req[28..34]), (0x20, &OTHER[..]));
    let element = [&[48u8, 38][..], &req[req.len() - 40 + 2..]].concat();
    assert_eq!(req[req.len() - 40..], element[..], "the RSNE, last");
    assert_eq!(req[req.len() - 16..], p.pmkid, "ending with the PMKID");
    m.on_mgmt(&mgmt(3, AP, STA, &[0x11, 0x04, 0, 0, 1, 0xc0]));
    four_way(&mut m, &p.pmk);
    assert!(m.used_pmksa() && m.pmksa() == Some(p));
}
