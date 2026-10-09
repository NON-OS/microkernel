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

//! Request decode on bytes from another capsule: no length or content may
//! panic it, and a header is taken only with NNET magic and version 1.

use crate::protocol::{decode_request, HDR_LEN};

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

const MAGIC: [u8; 4] = 0x4E4E_4554u32.to_le_bytes();

#[test]
fn hostile_buffers_never_panic_and_only_a_good_header_decodes() {
    for seed in 1..300_000u64 {
        let mut s = seed;
        let len = (xorshift(&mut s) % 64) as usize;
        let mut buf: Vec<u8> = (0..len).map(|_| xorshift(&mut s) as u8).collect();
        if seed % 3 == 0 && len >= 6 {
            buf[..4].copy_from_slice(&MAGIC);
            buf[4..6].copy_from_slice(&1u16.to_le_bytes());
        }
        let good = len >= HDR_LEN && buf[..4] == MAGIC && buf[4..6] == [1, 0];
        match decode_request(&buf) {
            None => assert!(!good, "a good header was refused"),
            Some(r) => {
                assert!(good, "a bad header was taken");
                assert_eq!(r.op, u16::from_le_bytes([buf[6], buf[7]]));
                assert_eq!(r.flags, u16::from_le_bytes([buf[8], buf[9]]));
                assert_eq!(r.request_id, u32::from_le_bytes(buf[12..16].try_into().unwrap()));
                assert_eq!(r.payload_len, u32::from_le_bytes(buf[16..20].try_into().unwrap()));
            }
        }
    }
}

#[test]
fn every_short_buffer_is_refused() {
    let full = [0xAAu8; HDR_LEN];
    for len in 0..HDR_LEN {
        assert!(decode_request(&full[..len]).is_none());
    }
}
