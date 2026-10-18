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

//! SLIP decoding and packet checks against a hostile line: noise before
//! the first delimiter, escape codes SLIP does not define, frames longer
//! than any packet, and packets whose length or CRC does not hold.

use nonos_bt_hci::uart::h5::header::Header;
use nonos_bt_hci::uart::h5::slip::encode;
use nonos_bt_hci::uart::h5::{build, open, Bad, Deslip, FRAME_MAX};

fn frames(d: &mut Deslip, line: &[u8]) -> Vec<Vec<u8>> {
    line.iter().filter_map(|&b| d.push(b)).collect()
}

#[test]
fn every_octet_comes_back_through_slip_with_or_without_oof() {
    let all: Vec<u8> = (0..=255).collect();
    for oof in [false, true] {
        let mut d = Deslip::default();
        d.oof = oof;
        assert_eq!(frames(&mut d, &encode(&all, oof)), std::slice::from_ref(&all));
    }
}

#[test]
fn noise_bad_escapes_and_overlong_frames_are_dropped_and_the_next_frame_taken() {
    let mut d = Deslip::default();
    let line = [0x55, 0x01, 0xc0, 0x01, 0xdb, 0x01, 0xc0, 0xc0, 0x02, 0xc0];
    assert_eq!(frames(&mut d, &line), [vec![0x02]]);
    assert_eq!(d.dropped, 1, "the frame with 0xDB 0x01");
    let mut d = Deslip::default();
    let xon = [0xc0, 0xdb, 0xde, 0xc0, 0xc0, 0x03, 0xc0];
    assert_eq!(frames(&mut d, &xon), [vec![0x03]], "0xDB 0xDE means nothing without OOF");
    let long = [&[0xc0][..], &vec![0x01; FRAME_MAX + 1], &[0xc0, 0x04, 0xc0]].concat();
    assert_eq!(frames(&mut Deslip::default(), &long), [vec![0x04]]);
}

#[test]
fn a_packet_is_taken_only_when_its_length_and_check_hold() {
    let h = Header { seq: 1, ack: 2, dic: true, reliable: true, kind: 4, len: 0 };
    let p = build(h, &[0x0e, 0x01, 0x05]).unwrap();
    assert_eq!(p.len(), 4 + 3 + 2);
    let (got, payload) = open(&p).unwrap();
    assert_eq!((got.len, payload), (3, &[0x0e, 0x01, 0x05][..]));
    for bit in 32..p.len() * 8 {
        let mut q = p.clone();
        q[bit / 8] ^= 1 << (bit % 8);
        assert_eq!(open(&q).map(|_| ()), Err(Bad::Check), "bit {bit}");
    }
    assert_eq!(open(&p[..p.len() - 1]).map(|_| ()), Err(Bad::Length));
    assert_eq!(open(&[&p[..], &[0]].concat()).map(|_| ()), Err(Bad::Length));
    assert_eq!(open(&p[..3]).map(|_| ()), Err(Bad::Header));
    assert!(build(h, &vec![0; 4096]).is_none(), "over the 12-bit length");
}
