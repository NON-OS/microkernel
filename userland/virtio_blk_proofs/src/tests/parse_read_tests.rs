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

//! The read parser receives attacker-controlled bytes over IPC: an LBA and
//! a sector count. An accepted read stays on the disk, and declares exactly
//! a header's payload. The parser takes the capacity alone, so no device
//! memory is near it.

use super::frames::{req, rw_body, xorshift};
use crate::constants::{MAX_SECTORS_PER_REQUEST, SECTOR_SIZE};
use crate::protocol::{E_INVAL, E_MSGSIZE, E_NXIO, RW_HEADER_LEN};
use crate::server::read::parse_read;

#[test]
fn read_parse_never_panics_and_requests_stay_within_the_disk() {
    for seed in 1..150_000u64 {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 20) as usize;
        let body: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        let capacity = xorshift(&mut s);
        let payload_len =
            if seed % 2 == 0 { RW_HEADER_LEN as u32 } else { xorshift(&mut s) as u32 };

        if let Ok((lba, nsectors, bytes_n)) = parse_read(capacity, &req(payload_len), &body) {
            assert!((1..=MAX_SECTORS_PER_REQUEST).contains(&nsectors));
            let last = lba.checked_add(nsectors as u64);
            assert!(last.is_some(), "lba + count overflowed");
            assert!(last.unwrap() <= capacity, "request reaches past the disk");
            assert_eq!(bytes_n, nsectors as usize * SECTOR_SIZE);
            assert_eq!(payload_len, RW_HEADER_LEN as u32);
        }
    }
}

#[test]
fn read_parse_enforces_the_exact_boundaries() {
    let cap = 1_000_000u64;
    let hdr = req(RW_HEADER_LEN as u32);
    let ok = |lba, n| parse_read(cap, &hdr, &rw_body(lba, n, 0));
    assert_eq!(ok(cap - 1, 1), Ok((cap - 1, 1, SECTOR_SIZE)));
    assert_eq!(ok(cap, 1).err(), Some(E_NXIO));
    let m = MAX_SECTORS_PER_REQUEST;
    assert!(ok(cap - m as u64, m).is_ok());
    assert_eq!(ok(cap - m as u64 + 1, m).err(), Some(E_NXIO));
    assert_eq!(ok(0, 0).err(), Some(E_INVAL));
    assert_eq!(ok(0, m + 1).err(), Some(E_INVAL));
    assert_eq!(ok(u64::MAX, 1).err(), Some(E_INVAL));
    // A short body or a mismatched declared payload length is a framing error.
    for len in 0..RW_HEADER_LEN {
        assert_eq!(parse_read(cap, &hdr, &vec![0u8; len]).err(), Some(E_MSGSIZE));
    }
    assert_eq!(parse_read(cap, &req(11), &rw_body(0, 1, 0)).err(), Some(E_MSGSIZE));
    assert_eq!(parse_read(cap, &req(13), &rw_body(0, 1, 0)).err(), Some(E_MSGSIZE));
}
