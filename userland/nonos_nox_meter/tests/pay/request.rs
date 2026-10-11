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

use nonos_nox_meter::pay::parse_pay;
use nonos_nox_meter::MeterError;

use super::fixture::body;

#[test]
fn every_field_sits_where_the_wire_puts_it() {
    let b = body(b"linux.qwen-small", b"qwen-small@1");
    let r = parse_pay(&b).unwrap();
    assert_eq!(
        (r.listing_id, r.release_hash, r.payee, r.unit),
        ([1; 32], [2; 32], [0xbb; 20], [3; 32])
    );
    assert_eq!((r.qty, r.unit_price, r.call_id, r.chain_id), (5, 7, [4; 32], 1));
    assert_eq!((r.listing, r.release), (&b"linux.qwen-small"[..], &b"qwen-small@1"[..]));
    // An empty release string is the market's default release.
    assert!(parse_pay(&body(b"x", b"")).is_ok());
}

#[test]
fn a_cut_a_tail_an_empty_listing_or_the_old_body_is_refused() {
    let b = body(b"linux.qwen-small", b"qwen-small@1");
    for n in 0..b.len() {
        assert_eq!(parse_pay(&b[..n]), Err(MeterError::Corrupt), "{n} bytes");
    }
    let mut long = b.clone();
    long.push(0);
    assert_eq!(parse_pay(&long), Err(MeterError::Corrupt));
    assert_eq!(parse_pay(&body(b"", b"r")), Err(MeterError::Corrupt));
    assert_eq!(parse_pay(&[0u8; 124]), Err(MeterError::Corrupt));
}
