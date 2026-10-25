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

//! The header, requests and replies, byte for byte.

use crate::errno::EINVAL;
use crate::header::{Header, HDR_LEN};
use crate::ops;
use crate::request::Request;

pub(super) fn hdr(op: u16, output: u16, len: u32) -> Header {
    Header { op, flags: 0, output, request_id: 0x0102_0304, payload_len: len }
}

#[test]
fn the_header_lands_on_section_12s_offsets() {
    let mut b = [0u8; 20];
    hdr(ops::FLUSH, 2, 72).encode(&mut b).unwrap();
    let want = [0x50, 0x47, 0x56, 0x4E, 1, 0, 0x14, 0, 0, 0, 2, 0, 4, 3, 2, 1, 72, 0, 0, 0];
    assert_eq!(b, want);
    assert_eq!(Header::decode(&b).unwrap(), hdr(ops::FLUSH, 2, 72));
}

#[test]
fn a_wrong_magic_version_or_short_header_is_refused() {
    let mut b = [0u8; 20];
    hdr(ops::CAPS, 0, 0).encode(&mut b).unwrap();
    for (i, v) in [(0, 0x51u8), (4, 2)] {
        let mut c = b;
        c[i] = v;
        assert_eq!(Header::decode(&c), Err(EINVAL));
    }
    for n in 0..HDR_LEN {
        assert_eq!(Header::decode(&b[..n]), Err(EINVAL), "{n} bytes");
    }
    assert_eq!(hdr(1, 0, 0).encode(&mut [0u8; 19]), Err(EINVAL));
}

#[test]
fn a_request_holds_exactly_its_declared_payload_up_to_256_bytes() {
    let mut f = [7u8; 300];
    hdr(ops::FLUSH, 0, 236).encode(&mut f).unwrap();
    assert_eq!(Request::decode(&f[..256]).unwrap().payload.len(), 236);
    hdr(ops::FLUSH, 0, 237).encode(&mut f).unwrap();
    assert_eq!(Request::decode(&f), Err(EINVAL), "257 bytes in all");
    hdr(ops::FLUSH, 0, 40).encode(&mut f).unwrap();
    assert_eq!(Request::decode(&f[..59]), Err(EINVAL), "payload cut short");
    hdr(ops::FLUSH, 0, u32::MAX).encode(&mut f).unwrap();
    assert_eq!(Request::decode(&f), Err(EINVAL));
}
