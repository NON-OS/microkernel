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

//! The station's `protect_mgmt`: the deauthentication a station sends under
//! management frame protection takes its packet number from the same space
//! as data, and needs the pairwise key in software.

use crate::protect_tests::{tk, AP, STA};
use nonos_wifi_core::dot11::ccmp::{decrypt, pn_and_key};
use nonos_wifi_core::station::{Ccmp, LinkStation};

#[test]
fn protect_mgmt_shares_the_packet_number_with_data() {
    let mut s = LinkStation::new(STA);
    s.associate(AP, Ccmp::SoftwareTxHwRx { tk: tk() });
    let eth = [&AP[..], &STA[..], &[0x08, 0x00], b"x"].concat();
    let pn_of = |f: &[u8]| pn_and_key(f, 24).expect("a CCMP header").0;
    let d1 = s.tx_frame(&eth).expect("data");
    let deauth =
        [&[0xC0, 0x00, 0x00, 0x00][..], &AP[..], &STA[..], &AP[..], &[0x00, 0x00, 0x03, 0x00]]
            .concat();
    let m = s.protect_mgmt(&deauth).expect("protected under the pairwise key");
    let d2 = s.tx_frame(&eth).expect("data");
    assert_eq!((pn_of(&d1), pn_of(&m), pn_of(&d2)), (1, 2, 3), "one packet number space");
    assert_ne!(m[1] & 0x40, 0, "the Protected bit is set");
    assert_eq!(decrypt(&m, &tk()).expect("verifies"), vec![0x03, 0x00], "reason 3 inside");
}

#[test]
fn protect_mgmt_needs_a_software_key() {
    let mut s = LinkStation::new(STA);
    s.associate(AP, Ccmp::Hardware);
    let deauth =
        [&[0xC0, 0x00, 0x00, 0x00][..], &AP[..], &STA[..], &AP[..], &[0x00, 0x00, 0x03, 0x00]]
            .concat();
    assert!(s.protect_mgmt(&deauth).is_none(), "no key in software, nothing to protect with");
}
