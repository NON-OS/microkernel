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

//! A deauthentication sent to all under BIP. The pinned frame (reason 3, key
//! index 4, IPN 2, under `IGTK`) was signed with the Python `cryptography`
//! AES-CMAC from the BIP rules, apart from this crate.

use super::frames::{bip_signed, mgmt, DEAUTH};
use super::sim::{h, joined, IGTK};
use nonos_wifi_core::dot11::bip::{bip_open, Mme};
use nonos_wifi_core::dot11::header::BROADCAST;
use nonos_wifi_core::station::{Loss, Rx, RxDrop};

const PINNED_BIP: &str = "c0000000 ffffffffffff 020000000001 020000000001 4001 0300
    4c10 0400 020000000000 e4e85b1919e6d71f";

#[test]
fn the_pinned_bip_frame_holds_and_any_change_breaks_it() {
    let pinned = h(PINNED_BIP);
    assert_eq!(bip_open(&pinned, &IGTK), Some(Mme { key_id: 4, ipn: 2 }));
    let mut retried = pinned.clone();
    retried[1] |= 0x38; // Retry, Power Management, More Data: outside the MIC
    assert!(bip_open(&retried, &IGTK).is_some());
    for i in 0..pinned.len() {
        let mut f = pinned.clone();
        f[i] ^= 0x01;
        // Duration and Sequence Control are outside the MIC, as BIP has them.
        let outside = matches!(i, 2 | 3 | 22 | 23);
        assert_eq!(bip_open(&f, &IGTK).is_some(), outside, "byte {i}");
    }
    assert_eq!(bip_open(&pinned, &[0u8; 16]), None, "another key");
    for n in 0..pinned.len() {
        assert_eq!(bip_open(&pinned[..n], &IGTK), None, "cut to {n}");
    }
}

#[test]
fn a_bip_deauthentication_ends_the_link_once() {
    let mut s = joined(true);
    assert!(matches!(s.receive(&h(PINNED_BIP), false), Rx::Lost(Loss::Left(3))));
    let mut s = joined(true);
    let deauth = mgmt(DEAUTH, BROADCAST, &[3, 0]);
    for (ipn, key, id) in [(1, IGTK, 4), (5, [9u8; 16], 4), (5, IGTK, 5)] {
        let r = s.receive(&bip_signed(&deauth, id, ipn, &key), false);
        assert!(matches!(r, Rx::Dropped(RxDrop::Unprotected)), "IPN {ipn} key index {id}");
    }
    assert!(matches!(s.receive(&deauth, false), Rx::Dropped(RxDrop::Unprotected)));
    assert!(s.is_associated());
    let signed = bip_signed(&deauth, 4, 7, &IGTK);
    assert!(matches!(s.receive(&signed, false), Rx::Lost(Loss::Left(3))));
}
