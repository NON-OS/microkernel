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

//! `LinkStation::receive` delivers the frames from the BSS: a protected
//! frame becomes ethernet, QoS or not, and a chip-decrypted frame loses its
//! CCMP header and MIC. The drops are in `receive::drop_tests`, the group
//! key in `receive::group_tests`, the CCMP vectors in `receive::ccmp_tests`.

use crate::receive::ccmp_tests::tk;
use crate::receive::chip_tests::chip;
use crate::receive::frame::{from_ap, station, SRC, STA};
use crate::receive::join::ip_body;
use nonos_wifi_core::station::{Rx, RxDrop};

#[test]
fn a_protected_frame_from_the_bss_becomes_ethernet() {
    let mut s = station();
    for qos in [false, true] {
        let f = from_ap(STA, &ip_body(b"payload"), Some(&tk()), if qos { 2 } else { 1 }, 0, qos);
        let Rx::Ethernet(eth) = s.receive(&f, false) else { panic!("delivered, qos={qos}") };
        assert_eq!(&eth[0..6], &STA, "destination");
        assert_eq!(&eth[6..12], &SRC, "source is the FromDS Address 3");
        assert_eq!(&eth[12..14], &[0x08, 0x00]);
        assert_eq!(&eth[14..], b"payload");
    }
}

#[test]
fn a_chip_decrypted_frame_loses_its_ccmp_header_and_mic() {
    let mut s = station();
    // The chip hands up the frame decrypted in place: its CCMP header and
    // MIC as they came, the plaintext between.
    let sealed = from_ap(STA, &ip_body(b"chip"), Some(&tk()), 5, 0, true);
    let f = chip(&sealed, &ip_body(b"chip"));
    let Rx::Ethernet(eth) = s.receive(&f, true) else { panic!("delivered") };
    assert_eq!(&eth[14..], b"chip", "no MIC bytes trail the payload");
    assert!(matches!(s.receive(&f, true), Rx::Dropped(RxDrop::Replay)), "replay checked too");
}
