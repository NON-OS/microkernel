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

//! NBTC's client half refuses what is not a v1 reply to the request asked.

use nonos_bt_proto::ops::{OP_BONDS, OP_LINK};
use nonos_bt_proto::{read_reply, request, Asked, Health, Link, HDR_LEN, PASSKEY_MAX};

fn reply(op: u16, id: u32, status: i32, data: &[u8]) -> Vec<u8> {
    let mut r = vec![0u8; 64];
    let n = request(&mut r, op, id, &[status.to_le_bytes().as_slice(), data].concat()).unwrap();
    r.truncate(n);
    r
}

#[test]
fn a_reply_is_read_only_for_its_own_request() {
    let r = reply(OP_LINK, 7, -16, &[1, 2, 3]);
    assert_eq!(read_reply(&r, OP_LINK, 7), Some((-16, &[1u8, 2, 3][..])));
    assert_eq!(read_reply(&r, OP_LINK, 8), None, "another request");
    assert_eq!(read_reply(&r, OP_BONDS, 7), None, "another op");
    assert_eq!(read_reply(&r[..r.len() - 1], OP_LINK, 7), None, "cut short");
    assert_eq!(read_reply(&r[..HDR_LEN], OP_LINK, 7), None, "no status");
    for (at, what) in [(0, "magic"), (4, "version"), (10, "reserved half-word")] {
        let mut bad = r.clone();
        bad[at] ^= 1;
        assert_eq!(read_reply(&bad, OP_LINK, 7), None, "{what}");
    }
    let mut short = [0u8; 21];
    assert_eq!(request(&mut short, OP_LINK, 1, &[1, 2]), None, "no room for the body");
}

#[test]
fn health_and_link_take_only_what_v1_defines() {
    assert_eq!(Health::read(&[1, 0, 1]), Some(Health { version: 1, controller: true }));
    assert_eq!(Health::read(&[1, 0, 2]), None);
    assert_eq!(Health::read(&[1, 0]), None);
    assert!(Link::read(&[5, 0b11_1111, 0xFF]).is_some());
    assert_eq!(Link::read(&[6, 0, 0xFF]), None, "a seventh state");
    assert_eq!(Link::read(&[1, 0b100_0000, 0xFF]), None, "a seventh flag");
    assert_eq!(Link::read(&[1, 0, 0xFF, 0]), None);
}

#[test]
fn a_passkey_is_six_digits_where_its_kind_has_one() {
    let w = |kind: u8, v: u32| [&[kind][..], &v.to_le_bytes()].concat();
    assert_eq!(Asked::read(&w(1, PASSKEY_MAX)), Some(Asked::Show(PASSKEY_MAX)));
    assert_eq!(Asked::read(&w(3, 0)), Some(Asked::Confirm(0)));
    assert_eq!(Asked::read(&w(1, PASSKEY_MAX + 1)), None);
    assert_eq!(Asked::read(&w(3, PASSKEY_MAX + 1)), None);
    assert_eq!(Asked::read(&w(0, 5)), None, "nothing asked carries no digits");
    assert_eq!(Asked::read(&w(2, 5)), None, "digits to type are the user's");
    assert_eq!(Asked::read(&w(4, 0)), None);
    for a in [Asked::None, Asked::Show(42), Asked::Enter, Asked::Confirm(7)] {
        assert_eq!(Asked::read(&a.write()), Some(a));
    }
}
