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

//! The station's GCMP-256 data path: what it sends opens under the 32-octet
//! pairwise key at the AP, what the AP sends under it is delivered once, a
//! replay or a frame under another key is refused, and a group frame opens
//! under the GCMP-256 group key its key index names.

use crate::ap_sim::{AA, SPA};
use crate::soft::frames::from_ds;
use nonos_wifi_core::dot11::gcmp::{decrypt, encrypt};
use nonos_wifi_core::rsn::TemporalKey;
use nonos_wifi_core::station::{Ccmp, LinkStation, Rx, RxDrop};

const TK: [u8; 32] = [0x42; 32];
const GROUP: [u8; 32] = [0x24; 32];
const LLC: [u8; 9] = [0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x99];

fn linked() -> LinkStation {
    let mut sta = LinkStation::new(SPA);
    sta.associate(AA, Ccmp::Gcmp256 { tk: TK });
    sta
}

fn eth_of(rx: Rx) -> Vec<u8> {
    match rx {
        Rx::Ethernet(e) => e,
        Rx::Handshake { .. } => panic!("a handshake"),
        Rx::Dropped(why) => panic!("dropped: {why:?}"),
    }
}

#[test]
fn what_the_station_sends_opens_under_the_pairwise_key() {
    let mut sta = linked();
    let eth = [&AA[..], &SPA, &[0x08, 0x00, 0x45]].concat();
    let f = sta.tx_frame(&eth).unwrap();
    assert_eq!((f[1] & 0x40, &f[24..26], f[27]), (0x40, &[1u8, 0][..], 0x20), "PN 1, ExtIV");
    assert_eq!(decrypt(&f, &TK).unwrap(), [0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x45]);
    assert_eq!(decrypt(&sta.tx_frame(&eth).unwrap(), &TK).map(|_| ()), Some(()));
}

#[test]
fn what_the_ap_sends_is_delivered_once() {
    let mut sta = linked();
    let down = encrypt(&from_ds(SPA, &LLC), 7, &TK).unwrap();
    assert_eq!(eth_of(sta.receive(&down, false)), [&SPA[..], &AA, &[0x08, 0x00, 0x99]].concat());
    assert!(matches!(sta.receive(&down, false), Rx::Dropped(RxDrop::Replay)));
    let forged = encrypt(&from_ds(SPA, &LLC), 8, &[0x43; 32]).unwrap();
    assert!(matches!(sta.receive(&forged, false), Rx::Dropped(RxDrop::Undecryptable)));
}

#[test]
fn a_group_frame_opens_under_the_gcmp_group_key() {
    let mut sta = linked();
    sta.set_group_key(1, TemporalKey::Gcmp256(GROUP), 0);
    let mut down = encrypt(&from_ds([0xff; 6], &LLC), 3, &GROUP).unwrap();
    // The key index sits outside the AAD, so it is set after sealing.
    down[27] |= 1 << 6;
    assert_eq!(eth_of(sta.receive(&down, false))[..6], [0xff; 6]);
}
