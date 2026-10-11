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

use nonos_settle_proto::{listing_reply, parse_listing_reply, Listed, LISTED_MAX, MAX_URL};

fn row(i: u8, uri: &[u8]) -> Listed {
    Listed {
        listing_id: [1; 32],
        release_id: [i; 32],
        release_hash: [i ^ 0x80; 32],
        bond_id: [7; 32],
        block: 1_000 + i as u64,
        uri: uri.to_vec(),
    }
}

#[test]
fn a_listing_reply_round_trips_at_most_sixteen_rows() {
    let rows: Vec<Listed> = (0..3).map(|i| row(i, b"https://h/r")).collect();
    let b = listing_reply(&rows);
    assert_eq!(b.len(), 1 + 3 * (106 + 11));
    assert_eq!(parse_listing_reply(&b, &[1; 32]), Some(rows));
    let many: Vec<Listed> = (0..20).map(|i| row(i, b"https://h/r")).collect();
    let b = listing_reply(&many);
    assert_eq!(b[0] as usize, LISTED_MAX);
    assert_eq!(parse_listing_reply(&b, &[1; 32]).unwrap(), many[..LISTED_MAX].to_vec());
    let longest = row(1, &[b'u'; MAX_URL]);
    assert_eq!(
        parse_listing_reply(&listing_reply(std::slice::from_ref(&longest)), &[1; 32]),
        Some(vec![longest])
    );
    assert_eq!(parse_listing_reply(&[0], &[1; 32]), Some(vec![]));
}

#[test]
fn a_listing_reply_reads_whole_or_not_at_all() {
    let b = listing_reply(&[row(1, b"https://h/r")]);
    let no = |b: &[u8], why: &str| assert_eq!(parse_listing_reply(b, &[1; 32]), None, "{why}");
    no(&b[..b.len() - 1], "a byte short");
    no(&[b.as_slice(), &[0]].concat(), "a trailing byte");
    no(&[], "nothing");
    let mut more = b.clone();
    more[0] = 17;
    no(&more, "seventeen rows");
    let raw = |len: usize| {
        let mut r = vec![1u8];
        r.extend_from_slice(&[0; 104]);
        r.extend_from_slice(&(len as u16).to_le_bytes());
        r.extend(std::iter::repeat_n(b'u', len));
        r
    };
    no(&raw(0), "an empty uri");
    no(&raw(MAX_URL + 1), "a uri past MAX_URL");
    assert_eq!(listing_reply(&[row(1, b"")]), [0], "never written");
}
