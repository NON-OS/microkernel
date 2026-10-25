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

//! The wire form of OP_SIGN_TYPED_DATA: exact length, zero reserved, a
//! bounded field count.

use crate::server::typed::request::{parse, HDR, MAX_FIELDS};
use crate::server::typed::table::entry;

pub fn request(pid: u32, key: u32, entry: u16, chain: u64, words: &[[u8; 32]]) -> Vec<u8> {
    let mut p = Vec::new();
    p.extend_from_slice(&pid.to_le_bytes());
    p.extend_from_slice(&key.to_le_bytes());
    p.extend_from_slice(&entry.to_le_bytes());
    p.extend_from_slice(&[0, 0]);
    p.extend_from_slice(&chain.to_le_bytes());
    p.push(words.len() as u8);
    words.iter().for_each(|w| p.extend_from_slice(w));
    p
}

#[test]
fn reads_every_field_where_the_plan_puts_it() {
    let p = request(7, 9, 2, 11_155_111, &[[1; 32], [2; 32], [3; 32]]);
    assert_eq!(p.len(), HDR + 96);
    let t = parse(&p).unwrap();
    assert_eq!((t.pid, t.key, t.entry, t.chain), (7, 9, 2, 11_155_111));
    assert_eq!(t.words, &p[HDR..]);
}

#[test]
fn refuses_a_count_that_disagrees_with_the_length() {
    let mut p = request(7, 9, 1, 1, &[[0; 32]; 2]);
    p.pop();
    assert!(parse(&p).is_none());
    p.push(0);
    p.push(0);
    assert!(parse(&p).is_none());
    assert!(parse(&p[..HDR - 1]).is_none());
}

#[test]
fn refuses_a_set_reserved_byte_and_too_many_fields() {
    let mut p = request(7, 9, 1, 1, &[]);
    p[10] = 1;
    assert!(parse(&p).is_none());
    let p = request(7, 9, 1, 1, &vec![[0; 32]; MAX_FIELDS + 1]);
    assert!(parse(&p).is_none());
    assert!(parse(&request(7, 9, 1, 1, &vec![[0; 32]; MAX_FIELDS])).is_some());
}

#[test]
fn only_the_three_entries_exist() {
    assert_eq!(entry(1).unwrap().typed.type_string.split('(').next(), Some("Voucher"));
    assert_eq!(entry(2).unwrap().typed.type_string.split('(').next(), Some("PublisherBinding"));
    assert_eq!(entry(3).unwrap().typed.type_string.split('(').next(), Some("ReleaseAttest"));
    for id in [0, 4, 11, 25, u16::MAX] {
        assert!(entry(id).is_none(), "{id}");
    }
}
