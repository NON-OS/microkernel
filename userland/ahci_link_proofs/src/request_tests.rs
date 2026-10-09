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

//! What a client sends the driver: the request header and the read/write
//! body. Both are another capsule's bytes, so neither may panic on anything,
//! and a block request must land inside the disk before any DMA is built.

use crate::constants::ata::MAX_SECTORS;
use crate::protocol::{decode_request, E_INVAL, E_MSGSIZE, E_NXIO, HDR_LEN, RW_HEADER_LEN};
use crate::rw_parse::parse;

const MAGIC: [u8; 4] = 0x4e41_4843u32.to_le_bytes();

fn header() -> [u8; HDR_LEN] {
    let mut h = [0u8; HDR_LEN];
    h[0..4].copy_from_slice(&MAGIC);
    h[4..6].copy_from_slice(&1u16.to_le_bytes());
    h
}

fn body(lba: u64, n: u32) -> [u8; RW_HEADER_LEN] {
    let mut b = [0u8; RW_HEADER_LEN];
    b[0..8].copy_from_slice(&lba.to_le_bytes());
    b[8..12].copy_from_slice(&n.to_le_bytes());
    b
}

#[test]
fn a_header_too_short_or_mistagged_is_refused() {
    let h = header();
    for len in 0..HDR_LEN {
        assert!(decode_request(&h[..len]).is_none(), "{len} bytes decoded");
    }
    assert!(decode_request(&h).is_some());
    let mut wrong_magic = h;
    wrong_magic[0] ^= 1;
    assert!(decode_request(&wrong_magic).is_none());
    let mut wrong_version = h;
    wrong_version[4] = 2;
    assert!(decode_request(&wrong_version).is_none());
}

#[test]
fn decode_never_panics_on_garbage() {
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
    for len in 0..64usize {
        let mut buf = vec![0u8; len];
        for b in buf.iter_mut() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            *b = state as u8;
        }
        if len >= 6 {
            buf[0..4].copy_from_slice(&MAGIC);
            buf[4..6].copy_from_slice(&1u16.to_le_bytes());
        }
        let _ = decode_request(&buf);
    }
}

#[test]
fn a_body_shorter_than_its_header_is_refused_at_every_length() {
    let b = body(0, 1);
    for len in 0..RW_HEADER_LEN {
        assert_eq!(parse(&b[..len], 1 << 20), Err(E_MSGSIZE), "{len} bytes parsed");
    }
}

#[test]
fn zero_or_too_many_sectors_are_refused() {
    assert_eq!(parse(&body(0, 0), 1 << 20), Err(E_INVAL));
    assert_eq!(parse(&body(0, MAX_SECTORS + 1), 1 << 20), Err(E_INVAL));
    assert_eq!(parse(&body(0, u32::MAX), 1 << 20), Err(E_INVAL));
    assert_eq!(parse(&body(0, MAX_SECTORS), 1 << 20), Ok((0, MAX_SECTORS)));
}

#[test]
fn a_request_that_wraps_or_runs_off_the_disk_is_refused() {
    let capacity = 1000u64;
    assert_eq!(parse(&body(u64::MAX, 1), capacity), Err(E_INVAL), "lba + n wrapped");
    assert_eq!(parse(&body(u64::MAX - 1, 8), capacity), Err(E_INVAL));
    assert_eq!(parse(&body(999, 2), capacity), Err(E_NXIO), "one sector past the end");
    assert_eq!(parse(&body(1000, 1), capacity), Err(E_NXIO));
    assert_eq!(parse(&body(999, 1), capacity), Ok((999, 1)), "the last sector is readable");
    assert_eq!(parse(&body(0, 1), 0), Err(E_NXIO), "a disk with no capacity has no sectors");
}
