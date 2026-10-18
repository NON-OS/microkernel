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

//! AUX requests, replies and the DPCD read and write around them.

use crate::dp::aux::{header, i2c_reply, native_reply, AuxError, Reply, NATIVE_READ};
use crate::dp::native::{read, write, RETRIES};
use crate::model::sink::sink;
use crate::model::sink_aux::AuxEnd;

#[test]
fn a_request_header_packs_command_address_and_length_less_one() {
    assert_eq!(header(NATIVE_READ, 0x00202, 6), [0x90, 0x02, 0x02, 5]);
    assert_eq!(header(0x8, 0xF_FFFF, 16), [0x8F, 0xFF, 0xFF, 15]);
    assert_eq!(header(0x9, 0x1_2345, 40)[3], 15, "at most 16 bytes");
}

#[test]
fn replies_decode_native_and_i2c_halves_apart() {
    assert_eq!(native_reply(0x00), Some(Reply::Ack));
    assert_eq!(native_reply(0x10), Some(Reply::Nack));
    assert_eq!(native_reply(0x20), Some(Reply::Defer));
    assert_eq!(native_reply(0x30), None);
    assert_eq!(i2c_reply(0x40), Some(Reply::Nack));
    assert_eq!(i2c_reply(0x80), Some(Reply::Defer));
}

#[test]
fn a_long_read_is_split_into_sixteen_byte_transactions() {
    let s = sink(vec![]);
    s.borrow_mut().dpcd[0x10..0x40].iter_mut().enumerate().for_each(|(i, b)| *b = i as u8);
    let mut out = [0u8; 40];
    read(&mut AuxEnd(s.clone()), 0x10, &mut out).unwrap();
    assert!(out.iter().enumerate().all(|(i, &b)| b == i as u8));
    assert_eq!(s.borrow().transfers, 3);
}

#[test]
fn defers_are_retried_and_a_nack_is_final() {
    let s = sink(vec![]);
    s.borrow_mut().defers = RETRIES - 1;
    write(&mut AuxEnd(s.clone()), 0x600, &[1]).unwrap();
    assert_eq!(s.borrow().dpcd[0x600], 1);
    s.borrow_mut().defers = RETRIES;
    assert_eq!(write(&mut AuxEnd(s.clone()), 0x600, &[2]), Err(AuxError::Refused));
    s.borrow_mut().defers = 0;
    s.borrow_mut().nack = true;
    let mut b = [0u8; 1];
    assert_eq!(read(&mut AuxEnd(s.clone()), 0, &mut b), Err(AuxError::Refused));
    assert_eq!(s.borrow().dpcd[0x600], 1, "nothing written after the refusal");
}
