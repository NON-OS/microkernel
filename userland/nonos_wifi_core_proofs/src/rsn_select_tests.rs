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

//! The choice made from an access point's RSNE: SAE whenever offered, never
//! PSK for a network saved as WPA3, no TKIP group cipher, and the station's
//! RSNE for WPA2-PSK is exactly `wpa::RSN_IE`.

use crate::rsn_tests::body;
use nonos_wifi_core::rsn::build::station_rsne;
use nonos_wifi_core::rsn::suite::CIPHER_TKIP;
use nonos_wifi_core::rsn::{parse_rsne, select, JoinPolicy, Pmf, SelectError};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::RSN_IE;

#[test]
fn sae_is_chosen_whenever_offered() {
    let mixed = parse_rsne(&body(4, &[4], &[2, 8], 0x0080)).unwrap();
    let s = select(&mixed, true, JoinPolicy::ANY).unwrap();
    assert_eq!((s.akm, s.pmf, s.sae_h2e), (Akm::Sae, Pmf::Capable, true));
    let only = parse_rsne(&body(4, &[4], &[8], 0x00C0)).unwrap();
    let s = select(&only, false, JoinPolicy::ANY).unwrap();
    assert_eq!((s.akm, s.pmf, s.sae_h2e), (Akm::Sae, Pmf::Required, false));
    // A caller without SAE's randomness falls back to PSK on a mixed network.
    assert_eq!(select(&mixed, true, JoinPolicy::PSK_ONLY).unwrap().akm, Akm::Psk);
}

#[test]
fn a_network_saved_as_wpa3_is_never_joined_with_psk() {
    let psk_only = parse_rsne(&body(4, &[4], &[2], 0)).unwrap();
    assert_eq!(select(&psk_only, false, JoinPolicy::WPA3_ONLY), Err(SelectError::Downgrade));
    assert_eq!(select(&psk_only, false, JoinPolicy::ANY).unwrap().akm, Akm::Psk);
    let sha256 = parse_rsne(&body(4, &[4], &[6], 0x0080)).unwrap();
    assert_eq!(select(&sha256, false, JoinPolicy::WPA3_ONLY), Err(SelectError::Downgrade));
    // SAE without management frame protection is not WPA3.
    let no_mfp = parse_rsne(&body(4, &[4], &[8], 0)).unwrap();
    assert_eq!(select(&no_mfp, false, JoinPolicy::WPA3_ONLY), Err(SelectError::BadProtection));
}

#[test]
fn unsupported_ciphers_and_akms_are_refused() {
    let tkip_group = parse_rsne(&body(2, &[4], &[2], 0)).unwrap();
    assert_eq!(tkip_group.group, CIPHER_TKIP);
    assert_eq!(select(&tkip_group, false, JoinPolicy::ANY), Err(SelectError::UnsupportedCipher));
    let tkip_only = parse_rsne(&body(2, &[2], &[2], 0)).unwrap();
    assert_eq!(select(&tkip_only, false, JoinPolicy::ANY), Err(SelectError::UnsupportedCipher));
    let enterprise = parse_rsne(&body(4, &[4], &[1], 0)).unwrap();
    assert_eq!(select(&enterprise, false, JoinPolicy::ANY), Err(SelectError::UnsupportedAkm));
    let mfpr_without_mfpc = parse_rsne(&body(4, &[4], &[2], 0x0040)).unwrap();
    assert_eq!(select(&mfpr_without_mfpc, false, JoinPolicy::ANY), Err(SelectError::BadProtection));
}

#[test]
fn the_station_rsne_for_wpa2_psk_is_the_shared_constant() {
    let psk = parse_rsne(&body(4, &[4], &[2], 0)).unwrap();
    let sel = select(&psk, false, JoinPolicy::ANY).unwrap();
    assert_eq!(station_rsne(&sel), RSN_IE, "the association request and message 2 agree");
    let sae = parse_rsne(&body(4, &[4], &[8], 0x00C0)).unwrap();
    let e = station_rsne(&select(&sae, true, JoinPolicy::ANY).unwrap());
    assert_eq!(e[19], 8, "AKM SAE");
    assert_eq!(u16::from_le_bytes([e[20], e[21]]), 0x00C0, "MFPC and MFPR");
    assert!(parse_rsne(&e[2..]).unwrap().akm_sae, "our own element parses back");
}
