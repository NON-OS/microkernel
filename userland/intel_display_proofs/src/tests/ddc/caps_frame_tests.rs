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

//! The capabilities messages: a request against a checksum worked by hand,
//! a reply believed only whole, every spoiled byte refused for what it is,
//! and the end of the string an empty reply.

use crate::ddc::caps_frame::{caps_request, fragment, CAPS_READ};
use crate::ddc::error::DdcError;
use crate::model::mccs::Mccs;
use crate::model::mccs_caps::caps_reply;

fn read(reply: &[u8]) -> [u8; CAPS_READ] {
    let mut r = [0xFFu8; CAPS_READ];
    r[..reply.len()].copy_from_slice(reply);
    r
}

#[test]
fn a_request_names_its_offset_high_byte_first() {
    // 0x6E ^ 0x51 ^ 0x83 ^ 0xF3 ^ 0x00 ^ 0x00 = 0x4F.
    assert_eq!(caps_request(0), [0x51, 0x83, 0xF3, 0x00, 0x00, 0x4F]);
    assert_eq!(caps_request(0x0120), [0x51, 0x83, 0xF3, 0x01, 0x20, 0x4F ^ 0x01 ^ 0x20]);
}

#[test]
fn a_whole_reply_gives_its_bytes_and_one_past_the_end_none() {
    let m = Mccs::with(&[(0x10, 70, 100)]);
    let text = crate::model::mccs_caps::caps_text(&m);
    assert_eq!(fragment(0, &read(&caps_reply(&m, 0, 0))), Ok(&text[..32]));
    let end = text.len() as u16;
    let [hi, lo] = end.to_be_bytes();
    assert_eq!(fragment(end, &read(&caps_reply(&m, hi, lo))), Ok(&[][..]));
}

#[test]
fn every_spoiled_byte_is_refused() {
    let good = caps_reply(&Mccs::with(&[(0x10, 70, 100)]), 0, 32);
    for at in 0..good.len() {
        let mut bad = good.clone();
        bad[at] ^= 0x24;
        let r = read(&bad);
        let got = fragment(32, &r);
        assert!(got.is_err(), "byte {at} spoiled and believed");
        let want = match at {
            0 | 2..=4 => DdcError::Reply,
            1 => DdcError::Length,
            _ => DdcError::Checksum,
        };
        assert!(got == Err(want) || got == Err(DdcError::Checksum), "byte {at}: {got:?}");
    }
    assert_eq!(fragment(0, &read(&good)), Err(DdcError::Reply), "another offset's reply");
    let mut long = good.clone();
    long[1] = 0x80 | 36;
    assert_eq!(fragment(32, &read(&long)), Err(DdcError::Length), "more than 32 bytes");
}
