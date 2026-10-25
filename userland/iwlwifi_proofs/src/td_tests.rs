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

//! W2c on iwlwifi's link reply: message 3's Transition Disable bitmap reaches
//! the client, which pins the network on bit 0; a link not associated sends
//! none; and a reply for a 32-octet SSID fits the client's link buffer whole.

use crate::client_join_wire::{parse_link, LinkEnded, TAIL_MAX};
use crate::join_wire::{encode_link_with, LinkInfo};
use nonos_wifi_core::link_history::LinkHistory;

/// The client's link buffer (nonos_wifi_client driver/link.rs `RESP_LEN`).
const CLIENT_RESP_LEN: usize = 10 + 8 + 32 + 1 + TAIL_MAX;

#[test]
fn the_transition_disable_bitmap_reaches_the_client_and_pins() {
    let mut out = [0u8; 80];
    let mut h = LinkHistory::NEW;
    h.joined(0);
    for (akm, td, pins) in [(2, 1, true), (2, 0, false), (8, 0, true), (6, 0x0E, false)] {
        let info = LinkInfo { ssid: b"home", akm };
        let n = encode_link_with(&mut out, Some(&info), &h, td).unwrap();
        let l = parse_link(&out[10..n]).unwrap();
        assert_eq!((l.transition_disable, l.pins_wpa3()), (td, pins), "akm {akm} td {td}");
    }
    h.lost();
    let n = encode_link_with(&mut out, None, &h, 1).unwrap();
    let l = parse_link(&out[10..n]).unwrap();
    assert_eq!((l.associated, l.transition_disable, l.pins_wpa3()), (false, 0, false));
}

#[test]
fn a_reply_for_a_32_octet_ssid_fits_the_client_s_buffer_whole() {
    let mut out = [0u8; 80];
    let mut h = LinkHistory::NEW;
    h.joined(0);
    h.left();
    let info = LinkInfo { ssid: &[b's'; 32], akm: 2 };
    let n = encode_link_with(&mut out, Some(&info), &h, 1).unwrap();
    assert_eq!(n, CLIENT_RESP_LEN, "the reply is as long as the buffer, no longer");
    let l = parse_link(&out[10..n]).unwrap();
    assert_eq!((l.ssid().len(), l.ended, l.transition_disable), (32, LinkEnded::Left, 1));
}
