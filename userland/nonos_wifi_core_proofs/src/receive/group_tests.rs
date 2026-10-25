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

//! The group key on the link: group frames decrypt under the GTK above its
//! receive sequence counter, the station's own broadcasts coming back are
//! dropped, and the AP's group key handshake is answered and its key taken.

use crate::ap_sim::{group1, gtk_kde, GTK};
use crate::hex::h;
use crate::receive::frame::{from_ap, STA};
use crate::receive::join::{eapol_body, ip_body, joined};
use nonos_wifi_core::dot11::ccmp::decrypt;
use nonos_wifi_core::station::{Rx, RxDrop};
use nonos_wifi_core::wpa::akm::Akm;

#[test]
fn group_frames_decrypt_under_the_gtk_above_its_rsc() {
    let (mut s, _) = joined();
    let bcast = [0xFF; 6];
    let old = from_ap(bcast, &ip_body(b"arp"), Some(&GTK), 9, 1, false);
    assert!(matches!(s.receive(&old, false), Rx::Dropped(RxDrop::Replay)), "at the RSC");
    let new = from_ap(bcast, &ip_body(b"arp"), Some(&GTK), 10, 1, false);
    assert!(matches!(s.receive(&new, false), Rx::Ethernet(_)));
    let wrong_index = from_ap(bcast, &ip_body(b"arp"), Some(&GTK), 11, 2, false);
    assert!(matches!(s.receive(&wrong_index, false), Rx::Dropped(RxDrop::Undecryptable)));
    let mut echo = from_ap(bcast, &ip_body(b"arp"), Some(&GTK), 12, 1, false);
    echo[16..22].copy_from_slice(&STA);
    assert!(matches!(s.receive(&echo, false), Rx::Dropped(RxDrop::NotForUs)), "our own");
}

#[test]
fn the_group_key_handshake_is_answered_on_the_link() {
    let (mut s, ptk) = joined();
    let next = [0x22u8; 16];
    let mut tkb = [0u8; 16];
    tkb.copy_from_slice(&ptk[32..48]);
    let g1 = group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &next));
    let f = from_ap(STA, &eapol_body(&g1), Some(&tkb), 1, 0, true);
    let Rx::Handshake { frame, group_key, pairwise_key } = s.receive(&f, false) else {
        panic!("handled")
    };
    assert_eq!(group_key, Some((2, next)), "the new key for the driver to install");
    assert!(pairwise_key.is_none(), "the pairwise key stays");
    let reply = frame.expect("group message 2");
    assert_ne!(reply[1] & 0x40, 0, "sent protected, as the message came");
    let plain = decrypt(&reply, &tkb).expect("under the pairwise key");
    assert_eq!(&plain[..8], &h("aaaa03000000888e")[..]);
    // Frames under the new key now decrypt.
    let bcast = from_ap([0xFF; 6], &ip_body(b"new"), Some(&next), 1, 2, false);
    assert!(matches!(s.receive(&bcast, false), Rx::Ethernet(_)));
}
