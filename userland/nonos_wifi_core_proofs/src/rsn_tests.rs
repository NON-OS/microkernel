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

//! The RSN negotiation: an access point's RSNE is parsed field by field with
//! its defaults, a malformed one is refused, SAE is chosen whenever offered,
//! a network saved as WPA3 is never joined with PSK, a TKIP group cipher is
//! refused, and the station's RSNE for WPA2-PSK is exactly `wpa::RSN_IE`. The
//! key data walk finds the RSNE, RSNXE, GTK and IGTK and stops at padding.

use nonos_wifi_core::eapol::kde::parse_key_data;
use nonos_wifi_core::rsn::build::station_rsne;
use nonos_wifi_core::rsn::rsnxe::advertises_h2e;
use nonos_wifi_core::rsn::suite::{CIPHER_BIP_CMAC_128, CIPHER_CCMP, CIPHER_TKIP};
use nonos_wifi_core::rsn::{parse_rsne, select, JoinPolicy, Pmf, SelectError};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::RSN_IE;

// An RSNE body: version 1, group, pairwise list, AKM list, caps.
fn body(group: u8, pairwise: &[u8], akms: &[u8], caps: u16) -> Vec<u8> {
    let mut b = vec![1, 0, 0x00, 0x0f, 0xac, group];
    b.extend_from_slice(&(pairwise.len() as u16).to_le_bytes());
    for p in pairwise {
        b.extend_from_slice(&[0x00, 0x0f, 0xac, *p]);
    }
    b.extend_from_slice(&(akms.len() as u16).to_le_bytes());
    for a in akms {
        b.extend_from_slice(&[0x00, 0x0f, 0xac, *a]);
    }
    b.extend_from_slice(&caps.to_le_bytes());
    b
}

#[test]
fn an_rsne_is_parsed_with_its_defaults() {
    let r = parse_rsne(&body(4, &[2, 4], &[2, 8], 0x00C0)).unwrap();
    assert_eq!(r.group, CIPHER_CCMP);
    assert!(r.pairwise_ccmp && r.akm_psk && r.akm_sae && !r.akm_psk_sha256);
    assert!(r.mfpc() && r.mfpr());
    // Only the version: CCMP everywhere and 802.1X, which this station lacks.
    let v = parse_rsne(&[1, 0]).unwrap();
    assert!(v.pairwise_ccmp && !v.akm_psk && !v.akm_sae);
    // The group management cipher after an empty PMKID list.
    let mut b = body(4, &[4], &[8], 0x00C0);
    b.extend_from_slice(&[0, 0, 0x00, 0x0f, 0xac, 6]);
    assert_eq!(parse_rsne(&b).unwrap().group_mgmt, Some(CIPHER_BIP_CMAC_128));
}

#[test]
fn a_malformed_rsne_is_refused() {
    assert!(parse_rsne(&[]).is_none(), "empty");
    assert!(parse_rsne(&[2, 0]).is_none(), "version 2");
    let mut b = body(4, &[4], &[2], 0);
    b[6] = 9; // pairwise count 9 with one suite present
    assert!(parse_rsne(&b).is_none(), "a list past the end");
    assert!(parse_rsne(&[1, 0, 0x00, 0x0f, 0xac]).is_none(), "a cut suite");
    let mut p = body(4, &[4], &[2], 0);
    p.extend_from_slice(&[3, 0, 1, 2]); // three PMKIDs announced, none present
    assert!(parse_rsne(&p).is_none(), "a PMKID list past the end");
}

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

#[test]
fn the_rsnxe_h2e_bit_is_read() {
    assert!(advertises_h2e(&[0x20]));
    assert!(!advertises_h2e(&[0x00]));
    assert!(!advertises_h2e(&[]));
}

#[test]
fn key_data_is_walked_to_its_padding() {
    let mut kd = vec![48, 2, 1, 0]; // a minimal RSNE
    kd.extend_from_slice(&[244, 1, 0x20]); // RSNXE
    kd.extend_from_slice(&[0xDD, 22, 0x00, 0x0f, 0xac, 0x01, 0x06, 0x00]);
    kd.extend_from_slice(&[0x5A; 16]);
    kd.extend_from_slice(&[0xDD, 0, 0, 0]); // padding
    let p = parse_key_data(&kd).unwrap();
    assert_eq!(p.rsne, Some(&[48u8, 2, 1, 0][..]));
    assert_eq!(p.rsnxe, Some(&[244u8, 1, 0x20][..]));
    let g = p.gtk.unwrap();
    assert_eq!((g.key_id, g.key), (2, &[0x5Au8; 16][..]), "only the index bits of 0x06");
    assert!(p.igtk.is_none());
    // A KDE that runs past the end is refused; padding that does is not.
    assert!(parse_key_data(&[0xDD, 10, 0x00, 0x0f, 0xac, 0x01]).is_none());
    assert!(parse_key_data(&[0xDD, 0x00, 0x00]).is_some());
    assert!(parse_key_data(&[0xDD, 22, 0x00, 0x0f, 0xac, 0x01, 1]).is_none(), "a cut GTK KDE");
}
