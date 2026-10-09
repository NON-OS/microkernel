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

//! Base64 as the live directory documents actually spell it.

use crate::directory::decode;
use crate::hex;

/*
 * From the microdescriptor consensus served by authority 49.13.145.234:9230 on
 * 2026-09-18. An identity on an `r` line is 27 characters with no padding; an
 * ntor-onion-key is 43. Both unpadded forms have to work or every relay in the
 * consensus is unreadable.
 */
#[test]
fn a_live_identity_is_twenty_bytes_unpadded() {
    let identity = decode(b"AAqxoECWZjs8pI8K+sp0l+t8IbE").expect("identity decodes");
    assert_eq!(identity.len(), 20);
    assert_eq!(identity, hex("000ab1a04096663b3ca48f0afaca7497eb7c21b1"));
}

#[test]
fn a_live_ntor_key_is_thirty_two_bytes_unpadded() {
    let key = decode(b"4k42YwphMtr96Q1qlyhy6O9RB6h9xzkmtxEMqd3mU3I").expect("key decodes");
    assert_eq!(key.len(), 32);
}

#[test]
fn a_wrapped_block_ignores_its_line_breaks() {
    assert_eq!(decode(b"Zm9v\nYmFy\n").as_deref(), Some(&b"foobar"[..]));
}
