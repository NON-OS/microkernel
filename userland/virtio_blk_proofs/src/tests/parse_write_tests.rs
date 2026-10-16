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

//! The write parser: the body carries the data, so the framing must be
//! exact, header length, body length and declared payload length agreeing
//! before a byte is copied toward the device.

use super::frames::{req, rw_body, xorshift};
use crate::constants::{MAX_SECTORS_PER_REQUEST, SECTOR_SIZE};
use crate::protocol::RW_HEADER_LEN;
use crate::server::write::parse_write;

#[test]
fn write_parse_never_panics_and_accepts_only_exact_frames() {
    for seed in 1..40_000u64 {
        let mut s = seed;
        let n = (xorshift(&mut s) % 70) as u32;
        let lba = xorshift(&mut s);
        let capacity = xorshift(&mut s);
        let mut body = rw_body(lba, n, n as usize * SECTOR_SIZE);
        // A third of the runs tamper with the frame.
        match seed % 3 {
            1 => body.push(0),
            2 => {
                body.pop();
            }
            _ => {}
        }
        let payload_len = if seed % 2 == 0 { body.len() as u32 } else { xorshift(&mut s) as u32 };

        if let Ok((plba, pn, bytes_n)) = parse_write(capacity, &req(payload_len), &body) {
            assert!((1..=MAX_SECTORS_PER_REQUEST).contains(&pn));
            assert_eq!(plba, lba);
            assert_eq!(bytes_n, pn as usize * SECTOR_SIZE);
            assert_eq!(body.len(), RW_HEADER_LEN + bytes_n, "accepted frame must be exact");
            assert_eq!(payload_len as usize, body.len());
            let last = plba.checked_add(pn as u64);
            assert!(last.is_some());
            assert!(last.unwrap() <= capacity);
        }
    }
}
