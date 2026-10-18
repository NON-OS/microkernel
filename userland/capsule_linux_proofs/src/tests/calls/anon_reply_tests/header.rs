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

//! The header, laid out where net.anon reads and writes it.

use super::values::{PARSE, RESPOND};
use crate::linux::net::anon_ops::*;
use crate::linux::net::anon_wire::{request, REQUEST_ID};

#[test]
fn the_header_is_laid_out_where_net_anon_reads_and_writes_it() {
    for line in [
        "if le32(buf, 0) != MAGIC",
        "if le16(buf, 4) != VERSION",
        "let payload_len = le32(buf, 16) as usize;",
        "Request { op: le16(buf, 6), request_id: le32(buf, 12) }",
    ] {
        assert!(PARSE.contains(line), "parse_req.rs no longer reads {line}");
    }
    for line in [
        "tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());",
        "tx[4..6].copy_from_slice(&VERSION.to_le_bytes());",
        "tx[6..8].copy_from_slice(&op.to_le_bytes());",
        "tx[8..10].copy_from_slice(&errno.to_le_bytes());",
        "tx[12..16].copy_from_slice(&request_id.to_le_bytes());",
        "tx[16..20].copy_from_slice(&payload_len.to_le_bytes());",
    ] {
        assert!(RESPOND.contains(line), "respond.rs no longer writes {line}");
    }
    let tx = request(OP_SEND, b"\x07\x00hello");
    assert_eq!(tx.len(), HDR_LEN + 7);
    assert_eq!(&tx[0..4], &MAGIC.to_le_bytes());
    assert_eq!(&tx[4..6], &VERSION.to_le_bytes());
    assert_eq!(&tx[6..8], &OP_SEND.to_le_bytes());
    assert_eq!(&tx[12..16], &REQUEST_ID.to_le_bytes());
    assert_eq!(&tx[16..20], &7u32.to_le_bytes());
    assert_eq!(&tx[HDR_LEN..], b"\x07\x00hello");
    /* Never one of the SOCKS front's markers, which net.anon tells apart by the first byte. */
    assert!(!matches!(tx[0], 0..=2));
}
