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

//! WPA3-SAE against published known answers. The exchange end to end is in
//! `sae::exchange_tests`, the refusals in `sae::refusal_tests` and
//! `sae::commit_tests`.
//!
//! Vector source: IEEE Std 802.11-2020, Annex J.10 (SAE test vectors); the
//! bytes were copied from hostapd's transcription in
//! `src/common/common_module_tests.c`, function
//! `sae_tests()`, fetched from https://w1.fi/cgit/hostap/plain/src/common/
//! common_module_tests.c (sha256 a75c0bc7e9069a2c443cfd225e52c912493c0255448
//! 606948483d3e9e3bcd9e3). The hunting-and-pecking part derives the PWE for
//! addresses 4d:3f:2f:ff:e3:87 / a5:d8:aa:95:8e:3c and password
//! "mekmitasdigoat", builds the commit from the given rand and mask, and from
//! the peer's commit derives the KCK, PMK and PMKID. The hash-to-element part
//! derives PT for SSID "byteme", that password and identifier "psk4internet",
//! and from it the PWE for 00:09:5b:66:ec:1e / 00:0b:6b:d9:02:46.
use crate::hex::h;
use crate::sae::vectors::{hnp_commit, KCK, LOCAL_COMMIT, PEER_COMMIT, PMK, PMKID, PW};
use nonos_wifi_core::sae::frame::{commit_body, parse_commit};
use nonos_wifi_core::sae::group::point_xy;
use nonos_wifi_core::sae::h2e::{derive_pt, pwe_from_pt};

#[test]
fn hunting_and_pecking_gives_the_annex_j10_commit() {
    let c = hnp_commit();
    let body = commit_body(&c.scalar(), &c.element().unwrap(), &[], false);
    assert_eq!(body, h(LOCAL_COMMIT), "group, commit-scalar and COMMIT-ELEMENT");
}

#[test]
fn the_annex_j10_peer_commit_gives_its_kck_pmk_and_pmkid() {
    let c = hnp_commit();
    let peer = parse_commit(&h(PEER_COMMIT)).expect("the peer commit parses");
    let keys = c.derive_keys(&peer.scalar, &peer.element).expect("K is not the identity");
    assert_eq!(keys.kck.to_vec(), h(KCK), "KCK");
    assert_eq!(keys.pmk.to_vec(), h(PMK), "PMK");
    assert_eq!(keys.pmkid.to_vec(), h(PMKID), "PMKID");
}

#[test]
fn hash_to_element_gives_the_annex_j10_pwe() {
    let pt = derive_pt(b"byteme", PW, Some(b"psk4internet")).expect("PT");
    let a = [0x00, 0x09, 0x5b, 0x66, 0xec, 0x1e];
    let b = [0x00, 0x0b, 0x6b, 0xd9, 0x02, 0x46];
    let pwe = pwe_from_pt(&pt, &a, &b).expect("PWE");
    let xy = point_xy(&pwe).unwrap();
    assert_eq!(
        xy[..32].to_vec(),
        h("c93049b9e64000f848201649e999f2b5c22dea69b5632c9df4d633b8aa1f6c1e"),
        "PWE.x"
    );
    assert_eq!(
        xy[32..].to_vec(),
        h("73634e94b53d82e7383a8d258199d9dc1a5ee8269d060382ccbf33e614ff59a0"),
        "PWE.y"
    );
    // The PWE does not depend on which station derives it.
    assert_eq!(point_xy(&pwe_from_pt(&pt, &b, &a).unwrap()).unwrap(), xy);
}
