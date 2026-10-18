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
//! A frame the chip says it decrypted: where the station holds the key, the
//! plaintext must seal again to the MIC that came, or it is refused and moves
//! no counter; a chip holding the key alone is taken at its word.

use crate::ap_sim::GTK;
use crate::receive::ccmp_tests::tk;
use crate::receive::frame::{from_ap, station, AP, STA};
use crate::receive::join::{ip_body, joined};
use nonos_wifi_core::station::{Ccmp, LinkStation, Rx, RxDrop};

/// A sealed QoS frame as a chip hands it up: its headers and MIC as they
/// came, the plaintext between.
pub fn chip(sealed: &[u8], plain: &[u8]) -> Vec<u8> {
    [&sealed[..26 + 8], plain, &sealed[sealed.len() - 8..]].concat()
}

fn at(pn: u64, dst: [u8; 6], key: &[u8; 16], kid: u8) -> Vec<u8> {
    chip(&from_ap(dst, &ip_body(b"chip"), Some(key), pn, kid, true), &ip_body(b"chip"))
}

fn refused(r: Rx) -> bool {
    matches!(r, Rx::Dropped(RxDrop::Undecryptable))
}

#[test]
fn a_chips_word_that_does_not_seal_again_is_refused_and_moves_no_counter() {
    let mut s = station();
    let honest = at(1000, STA, &tk(), 0);
    for i in 34..honest.len() {
        let mut lie = honest.clone();
        lie[i] ^= 0x01;
        assert!(refused(s.receive(&lie, true)), "octet {i} changed");
    }
    let mut lie = honest.clone();
    lie[26] ^= 0x01;
    assert!(refused(s.receive(&lie, true)), "another packet number in its header");
    assert!(matches!(s.receive(&at(6, STA, &tk(), 0), true), Rx::Ethernet(_)), "no counter moved");
    assert!(matches!(s.receive(&honest, true), Rx::Ethernet(_)));
    assert!(matches!(s.receive(&honest, true), Rx::Dropped(RxDrop::Replay)));
}

#[test]
fn a_group_frame_said_decrypted_is_checked_under_the_group_key() {
    let (mut s, _) = joined();
    let honest = at(50, [0xff; 6], &GTK, 1);
    let mut lie = honest.clone();
    lie[40] ^= 0x80;
    assert!(refused(s.receive(&lie, true)));
    assert!(matches!(s.receive(&honest, true), Rx::Ethernet(_)), "the counter was not moved");
}

#[test]
fn a_chip_holding_the_key_alone_is_taken_at_its_word_with_replay_held() {
    let mut s = LinkStation::new(STA);
    s.associate(AP, Ccmp::Hardware);
    let f = at(5, STA, &[0x5a; 16], 0);
    assert!(matches!(s.receive(&f, true), Rx::Ethernet(_)), "no key to check it by");
    assert!(matches!(s.receive(&f, true), Rx::Dropped(RxDrop::Replay)));
    assert!(matches!(s.receive(&at(4, STA, &[0x5a; 16], 0), true), Rx::Dropped(RxDrop::Replay)));
}
