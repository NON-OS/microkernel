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

//! A connected SAE join leaves a PMKSA, and a later join offered it skips
//! SAE (IEEE Std 802.11-2020, 12.6.10.3): Open System authentication, the
//! PMKID in the association request's RSNE and again in message 2, and the
//! four-way handshake under the cached PMK.

use super::fixture::twin;
use super::sim::{accepted, by_sae, four_way, offered};
use crate::mlme::sim::AP;
use nonos_wifi_core::dot11::header::MAC_HEADER_LEN;

#[test]
fn a_connected_sae_join_holds_its_pmksa() {
    let (m, pmk) = by_sae();
    let p = m.pmksa().expect("a PMKSA");
    assert_eq!((p.bssid, p.pmk), (AP, pmk));
    assert_ne!(p.pmkid, [0u8; 16]);
    assert!(!m.used_pmksa());
}

#[test]
fn a_join_offered_the_pmksa_skips_sae_and_names_the_pmkid() {
    let (first, _) = by_sae();
    let p = first.pmksa().unwrap();
    let (mut m, auth, rsne) = offered(twin(&p));
    let body = &auth[MAC_HEADER_LEN..];
    assert_eq!(body[..6], [0, 0, 1, 0, 0, 0], "Open System, transaction 1, status 0");
    assert_eq!(rsne.len(), 38, "the 20-octet body, a PMKID count and one PMKID");
    assert_eq!(rsne[17], 8, "AKM SAE");
    assert_eq!(rsne[20..22], [1, 0], "PMKID count 1");
    assert_eq!(rsne[22..], p.pmkid);
    accepted(&mut m);
    assert!(m.pmksa().is_none(), "nothing to cache before the link is up");
    let m2 = four_way(&mut m, &p.pmk);
    let element = [&[48u8, 38][..], &rsne].concat();
    assert!(
        m2.windows(element.len()).any(|w| w == element),
        "message 2 repeats the request's RSNE"
    );
    assert!(m.used_pmksa());
    assert!(m.pmksa() == Some(p), "the same PMKSA, kept for the next join");
}
