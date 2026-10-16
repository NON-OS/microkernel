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

use crate::constants::{MAX_SECTORS_PER_REQUEST, SECTOR_SIZE};
use crate::protocol::{Request, RW_HEADER_LEN};
use crate::server::{read::parse_read, write::parse_write};

fn any_request() -> Request {
    Request {
        op: kani::any(),
        flags: kani::any(),
        request_id: kani::any(),
        payload_len: kani::any(),
    }
}

// For every body up to 16 bytes, every capacity, and every declared payload
// length: read parsing is total, and an accepted read stays on the disk with
// an exactly declared header length.
#[kani::proof]
fn read_parse_is_total_and_bounded() {
    let body: [u8; 16] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= body.len());
    let capacity: u64 = kani::any();
    let req = any_request();

    if let Ok((lba, nsectors, bytes_n)) = parse_read(capacity, &req, &body[..len]) {
        assert!(nsectors >= 1 && nsectors <= MAX_SECTORS_PER_REQUEST);
        let last = lba.checked_add(nsectors as u64);
        assert!(last.is_some());
        assert!(last.unwrap() <= capacity);
        assert!(bytes_n == nsectors as usize * SECTOR_SIZE);
        assert!(req.payload_len == RW_HEADER_LEN as u32);
    }
}

// For every body up to one sector plus header, every capacity, and every
// declared payload length: write parsing is total, and an accepted write is
// exactly framed: body and declared length both equal header plus data, the
// sector count is bounded, and the request stays on the disk.
#[kani::proof]
fn write_parse_is_total_and_exactly_framed() {
    let body: [u8; RW_HEADER_LEN + SECTOR_SIZE] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= body.len());
    let capacity: u64 = kani::any();
    let req = any_request();

    if let Ok((lba, nsectors, bytes_n)) = parse_write(capacity, &req, &body[..len]) {
        assert!(nsectors >= 1 && nsectors <= MAX_SECTORS_PER_REQUEST);
        assert!(bytes_n == nsectors as usize * SECTOR_SIZE);
        assert!(len == RW_HEADER_LEN + bytes_n);
        assert!(req.payload_len as usize == len);
        let last = lba.checked_add(nsectors as u64);
        assert!(last.is_some());
        assert!(last.unwrap() <= capacity);
    }
}
