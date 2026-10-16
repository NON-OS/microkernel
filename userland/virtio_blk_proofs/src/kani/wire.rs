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

use crate::protocol::{decode_request, read_u32_le, read_u64_le};

// For every buffer up to 16 bytes and every offset: the field readers never
// panic, and a successful read lies entirely inside the buffer even when
// offset plus width would overflow a usize.
#[kani::proof]
fn field_readers_are_total_and_inbounds() {
    let buf: [u8; 16] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= buf.len());
    let offset: usize = kani::any();

    if let Some(v) = read_u32_le(&buf[..len], offset) {
        assert!(offset + 4 <= len);
        assert_eq!(
            v,
            u32::from_le_bytes([buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3]])
        );
    }
    if read_u64_le(&buf[..len], offset).is_some() {
        assert!(offset + 8 <= len);
    }
}

// For every buffer up to a header plus slack: header decoding is total and
// every accepted field comes from its wire offset in little-endian order.
#[kani::proof]
fn decode_is_total_and_header_faithful() {
    let buf: [u8; 24] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= buf.len());

    if let Some(req) = decode_request(&buf[..len]) {
        assert!(len >= 20);
        assert_eq!(req.op, u16::from_le_bytes([buf[6], buf[7]]));
        assert_eq!(req.flags, u16::from_le_bytes([buf[8], buf[9]]));
        assert_eq!(req.request_id, u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]));
        assert_eq!(req.payload_len, u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]));
    }
}

// For every op, sender and StoreWrite verdict: the medium, reads included,
// is served only to the kernel client or a StoreWrite holder, and a health
// check is never refused.
#[kani::proof]
fn medium_authority_is_exactly_the_kernel_or_a_storewrite_holder() {
    use crate::protocol::OP_HEALTHCHECK;
    use crate::server::acl::rule::allows;
    let op: u16 = kani::any();
    let pid: u32 = kani::any();
    let may_write: bool = kani::any();
    let verdict = allows(op, pid, may_write);
    if op == OP_HEALTHCHECK {
        assert!(verdict);
    } else {
        assert_eq!(verdict, pid == 0 || may_write);
    }
}
