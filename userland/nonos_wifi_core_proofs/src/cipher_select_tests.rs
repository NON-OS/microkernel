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

//! Cipher negotiation: GCMP-256 only where the access point's RSNE and the
//! radio's cipher set both offer it, CCMP-128 otherwise; a radio that says
//! nothing keeps CCMP-128, so the constant policies join exactly as before;
//! the station's RSNE names what was chosen.

use crate::cipher_sim::{GCMP_ONLY, MIXED};
use crate::soft::frames::RSNE_PSK;
use nonos_wifi_core::rsn::build::station_rsne;
use nonos_wifi_core::rsn::Cipher::{Ccmp128, Gcmp256};
use nonos_wifi_core::rsn::{parse_rsne, select, Ciphers, JoinPolicy, Rsne, SelectError};
use nonos_wifi_core::wpa::RSN_IE;

const GCMP: Ciphers = Ciphers { gcmp256: true };

fn rsne(e: &[u8]) -> Rsne {
    parse_rsne(&e[2..]).unwrap()
}

#[test]
fn the_element_lists_both_pairwise_ciphers() {
    let r = rsne(&MIXED);
    assert!(r.pairwise_ccmp && r.pairwise_gcmp256);
    assert!(!rsne(&RSNE_PSK).pairwise_gcmp256);
}

#[test]
fn a_radio_that_does_not_say_keeps_ccmp_128() {
    for p in [JoinPolicy::ANY, JoinPolicy::WPA3_ONLY, JoinPolicy::PSK_ONLY] {
        assert_eq!(p.ciphers, Ciphers::CCMP_128_ONLY);
    }
    let s = select(&rsne(&MIXED), false, JoinPolicy::ANY).unwrap();
    assert_eq!((s.pairwise, s.group), (Ccmp128, Ccmp128));
    assert_eq!(station_rsne(&s), RSN_IE, "the element is byte for byte the old one");
}

#[test]
fn gcmp_256_is_chosen_when_both_offer_it() {
    let s = select(&rsne(&MIXED), false, JoinPolicy::ANY.with_ciphers(GCMP)).unwrap();
    assert_eq!((s.pairwise, s.group), (Gcmp256, Ccmp128));
    let e = station_rsne(&s);
    assert_eq!((e[7], e[13]), (4, 9), "group CCMP-128, pairwise GCMP-256");
}

#[test]
fn ccmp_128_is_chosen_when_the_ap_lacks_gcmp() {
    let s = select(&rsne(&RSNE_PSK), false, JoinPolicy::ANY.with_ciphers(GCMP)).unwrap();
    assert_eq!((s.pairwise, s.group), (Ccmp128, Ccmp128));
}

#[test]
fn a_gcmp_only_network_needs_a_radio_that_runs_it() {
    let r = rsne(&GCMP_ONLY);
    assert_eq!(select(&r, false, JoinPolicy::ANY), Err(SelectError::UnsupportedCipher));
    let s = select(&r, false, JoinPolicy::ANY.with_ciphers(GCMP)).unwrap();
    assert_eq!((s.pairwise, s.group), (Gcmp256, Gcmp256));
}
