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
//! W1 (c): the link reply's history, read by the client at every length. A
//! reply from a driver that predates it reads as lost, never as left, and as
//! no join; a cut or garbled one never panics and is never read as left.

use crate::wifi::join_wire::{parse_link, LastJoin, LinkEnded};

fn reply(associated: bool, tail: &[u8]) -> Vec<u8> {
    let mut b = vec![u8::from(associated), 2, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE];
    let ssid: &[u8] = if associated { b"Home" } else { b"" };
    b.push(ssid.len() as u8);
    b.extend_from_slice(ssid);
    b.push(if associated { 8 } else { 0 });
    b.extend_from_slice(tail);
    b
}

#[test]
fn the_history_reads_at_the_new_length() {
    let left = parse_link(&reply(false, &[2, 3, 0xF6, 0xFF, 0xFF, 0xFF])).unwrap();
    assert!(!left.associated);
    assert_eq!(left.ended, LinkEnded::Left);
    assert_eq!(left.last_join, Some(LastJoin { seq: 3, code: -10 }));
    let up = parse_link(&reply(true, &[0, 1, 0, 0, 0, 0])).unwrap();
    assert_eq!((up.ssid(), up.akm, up.ended), (&b"Home"[..], 8, LinkEnded::No));
    assert_eq!(up.last_join, Some(LastJoin { seq: 1, code: 0 }));
    let none = parse_link(&reply(false, &[1, 0, 9, 9, 9, 9])).unwrap();
    assert_eq!((none.ended, none.last_join), (LinkEnded::Lost, None), "joins 0: none yet");
}

#[test]
fn an_old_reply_reads_as_lost_and_no_join() {
    for old in [reply(false, &[])[..8].to_vec(), reply(false, &[]), reply(true, &[])] {
        let l = parse_link(&old).unwrap();
        assert_eq!((l.ended, l.last_join), (LinkEnded::Lost, None), "{} bytes", old.len());
    }
}

#[test]
fn a_cut_or_garbled_history_is_never_read_as_left() {
    let full = reply(true, &[2, 5, 1, 0, 0, 0]);
    for n in 0..full.len() {
        if let Some(l) = parse_link(&full[..n]) {
            assert_eq!(l.ended, LinkEnded::Lost, "cut at {n}");
        }
    }
    for b in 3..=u8::MAX {
        let l = parse_link(&reply(false, &[b, 0, 0, 0, 0, 0])).unwrap();
        assert_eq!(l.ended, LinkEnded::Lost, "ended byte {b}");
    }
}
