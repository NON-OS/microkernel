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

//! A reply is believed only whole: the model monitor's own reply parses
//! to its values, and every single byte changed, a short or long length
//! byte, a reply about another control, "unsupported" and the null
//! message are each refused for what they are.

use crate::ddc::error::DdcError;
use crate::ddc::reply::{parse, Reading, REPLY_LEN};
use crate::model::mccs_reply::get_reply;

fn reply(controls: &[(u8, u16, u16)], code: u8) -> [u8; REPLY_LEN] {
    get_reply(controls, code).try_into().unwrap()
}

#[test]
fn the_monitors_reply_reads_as_its_value_and_maximum() {
    let r = reply(&[(0x10, 70, 100)], 0x10);
    assert_eq!(parse(0x10, &r), Ok(Reading { present: 70, max: 100 }));
    let r = reply(&[(0x12, 0x1234, 0xFFFF)], 0x12);
    assert_eq!(parse(0x12, &r), Ok(Reading { present: 0x1234, max: 0xFFFF }));
}

#[test]
fn any_one_byte_changed_is_refused() {
    let good = reply(&[(0x10, 70, 100)], 0x10);
    for i in 0..REPLY_LEN {
        for flip in [0x01u8, 0x80, 0xFF] {
            let mut r = good;
            r[i] ^= flip;
            assert!(parse(0x10, &r).is_err(), "byte {i} xor {flip:#x} was believed");
        }
    }
}

#[test]
fn each_refusal_is_the_one_it_should_be() {
    let good = reply(&[(0x10, 70, 100)], 0x10);
    assert_eq!(parse(0x12, &good), Err(DdcError::Reply), "an answer about another control");
    let mut long = good;
    (long[1], long[10]) = (0x89, long[10] ^ 0x88 ^ 0x89);
    assert_eq!(parse(0x10, &long), Err(DdcError::Length));
    assert_eq!(parse(0x10, &reply(&[], 0x10)), Err(DdcError::NotOffered), "result 1");
    let mut null = [0u8; REPLY_LEN];
    null[..3].copy_from_slice(&[0x6E, 0x80, 0x50 ^ 0x6E ^ 0x80]);
    assert_eq!(parse(0x10, &null), Err(DdcError::NotOffered), "the null message");
    null[2] ^= 1;
    assert_eq!(parse(0x10, &null), Err(DdcError::Checksum));
    let mut other = good;
    (other[0], other[10]) = (0x6F, other[10] ^ 0x6E ^ 0x6F);
    assert_eq!(parse(0x10, &other), Err(DdcError::Reply), "not from the display");
}
