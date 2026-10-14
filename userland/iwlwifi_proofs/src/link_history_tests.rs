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
//! W1 (c) on iwlwifi's link reply, written by the driver's encoder and read by
//! the client's parser: a lost link, a person's leave and the last join's
//! code all reach net.core, and the reply without its history (the old
//! length) still reads as lost.

use crate::client_join_wire::{parse_link, LastJoin, LinkEnded};
use crate::join_wire::{encode_link, encode_link_with, parse_connect, LinkInfo};
use nonos_wifi_core::link_history::LinkHistory;

const AP: [u8; 6] = [0x02, 0, 0, 0, 0, 0xAA];

#[test]
fn the_driver_s_history_reaches_the_client_whole() {
    let mut out = [0u8; 80];
    let mut h = LinkHistory::NEW;
    h.joined(0);
    let info = LinkInfo { bssid: AP, ssid: b"home", akm: 8 };
    let n = encode_link_with(&mut out, Some(&info), &h, 0).unwrap();
    let l = parse_link(&out[10..n]).unwrap();
    assert_eq!((l.ssid(), l.akm, l.ended), (&b"home"[..], 8, LinkEnded::No));
    assert_eq!(l.last_join, Some(LastJoin { seq: 1, code: 0 }));
    h.lost();
    let n = encode_link_with(&mut out, None, &h, 0).unwrap();
    let l = parse_link(&out[10..n]).unwrap();
    assert_eq!((l.associated, l.ended), (false, LinkEnded::Lost));
    h.joined(-10);
    h.left();
    let n = encode_link_with(&mut out, None, &h, 0).unwrap();
    let l = parse_link(&out[10..n]).unwrap();
    assert_eq!((l.ended, l.last_join), (LinkEnded::Left, Some(LastJoin { seq: 2, code: -10 })));
    assert!(encode_link_with(&mut out[..n - 1], None, &h, 0).is_none(), "no room, refused");
}

#[test]
fn the_reply_without_its_history_reads_as_lost() {
    let mut out = [0u8; 80];
    for info in [None, Some(LinkInfo { bssid: AP, ssid: b"home", akm: 2 })] {
        let n = encode_link(&mut out, info.as_ref()).unwrap();
        let l = parse_link(&out[10..n]).unwrap();
        assert_eq!((l.ended, l.last_join), (LinkEnded::Lost, None));
    }
}

#[test]
fn a_rejoin_is_flagged_in_the_connect_body_and_nothing_else_is() {
    for (flags, rejoin) in
        [(Some(4u8), true), (Some(4 | 1 | 2), true), (Some(3), false), (None, false)]
    {
        let mut b = vec![4, b'h', b'o', b'm', b'e', 2, b'p', b'w'];
        b.extend(flags);
        assert_eq!(parse_connect(&b).unwrap().rejoin, rejoin, "{flags:?}");
    }
}
