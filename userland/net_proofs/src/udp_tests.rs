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

//! net.udp's datagram builder and parser, against each other and noise.

use crate::udp::{build, parse, BuildRequest, HDR_LEN};

extern crate alloc;
use alloc::vec::Vec;

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

fn addr(s: &mut u32) -> [u8; 4] {
    xorshift(s).to_be_bytes()
}

/// Whatever the builder writes, the parser reads back exactly, and its
/// checksum field is never zero: zero means "no checksum" (RFC 768).
#[test]
fn every_built_datagram_parses_back_to_its_parts() {
    let mut s = 0x0D0D_0001u32;
    for _ in 0..100_000 {
        let (src, dst) = (addr(&mut s), addr(&mut s));
        let (sp, dp) = (xorshift(&mut s) as u16, xorshift(&mut s) as u16);
        let payload: Vec<u8> = (0..xorshift(&mut s) % 200).map(|_| xorshift(&mut s) as u8).collect();
        let mut out = alloc::vec![0u8; HDR_LEN + payload.len()];
        let req = BuildRequest { src, dst, src_port: sp, dst_port: dp, payload: &payload };
        let n = build(&req, &mut out).ok().expect("fits");
        assert_ne!(&out[6..8], &[0, 0], "a computed checksum is never sent as zero");
        let Ok((h, body)) = parse(&src, &dst, &out[..n]) else { panic!("parses back") };
        assert_eq!((h.src_port, h.dst_port, body), (sp, dp, payload.as_slice()));
    }
}

/// One flipped bit anywhere, or the wrong pseudo-header, is caught.
#[test]
fn a_damaged_datagram_or_the_wrong_addresses_are_caught() {
    let mut s = 0x0D0D_0002u32;
    for _ in 0..20_000 {
        let (src, dst) = (addr(&mut s), addr(&mut s));
        let payload: Vec<u8> = (0..1 + xorshift(&mut s) % 64).map(|_| xorshift(&mut s) as u8).collect();
        let mut out = alloc::vec![0u8; HDR_LEN + payload.len()];
        let req = BuildRequest { src, dst, src_port: 53, dst_port: 4444, payload: &payload };
        let n = build(&req, &mut out).ok().expect("fits");
        let bit = xorshift(&mut s) as usize % (n * 8);
        let mut bad = out[..n].to_vec();
        bad[bit / 8] ^= 1 << (bit % 8);
        let flipped_checksum_to_zero = bad[6] == 0 && bad[7] == 0;
        assert!(parse(&src, &dst, &bad).is_err() || flipped_checksum_to_zero, "bit {bit}");
        let mut other = dst;
        other[3] ^= 1;
        assert!(parse(&src, &other, &out[..n]).is_err(), "the pseudo-header is covered");
    }
}

#[test]
fn lengths_outside_the_segment_are_refused() {
    let mut d = alloc::vec![0, 53, 0, 54, 0, 0, 0, 0, 1, 2, 3];
    for len in [0u16, 7, 12, 0xFFFF] {
        d[4..6].copy_from_slice(&len.to_be_bytes());
        assert!(parse(&[1, 1, 1, 1], &[2, 2, 2, 2], &d).is_err(), "length {len}");
    }
    d[4..6].copy_from_slice(&9u16.to_be_bytes());
    let Ok((_, body)) = parse(&[1, 1, 1, 1], &[2, 2, 2, 2], &d) else { panic!("no checksum") };
    assert_eq!(body, &[1], "the UDP length, not the IP payload, ends the data");
}

#[test]
fn random_bytes_never_panic_and_stay_in_bounds() {
    let mut s = 0x0D0D_0003u32;
    for _ in 0..100_000 {
        let seg: Vec<u8> = (0..xorshift(&mut s) % 64).map(|_| xorshift(&mut s) as u8).collect();
        if let Ok((_, body)) = parse(&[10, 0, 0, 1], &[10, 0, 0, 2], &seg) {
            let (s0, b0) = (seg.as_ptr() as usize, body.as_ptr() as usize);
            assert!(b0 == s0 + HDR_LEN && b0 + body.len() <= s0 + seg.len());
        }
    }
}
