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

//! Where the record lies and what it is bound to: it round-trips, a clear
//! leaves every other byte of the sector, a header changed in any byte way 3
//! binds does not open it, and its sealed bytes put in way 3's place do not
//! open there, even under the same key.

use super::super::key_header::{parse_key_header, Keyed, Sealed};
use super::super::key_seal::encode_key_header;
use super::super::key_wrap::{unwrap as unwrap3, wrap as wrap3};
use super::place::{put, read, Record};
use super::seal::{unwrap, wrap};
use crate::crypto::util::argon2::Params;

pub(super) const OWN: [u8; 32] = [7; 32];
pub(super) const HAND: [u8; 32] = [8; 32];
pub(super) const KEY: [u8; 32] = [42; 32];

/// A way-3 header under `OWN` with a record under `HAND`.
pub(super) fn prepared() -> ([u8; 512], Record) {
    let s = encode_key_header(&Keyed::Approved(wrap3(&OWN, &KEY, [9; 12]).unwrap()));
    let r = wrap(&HAND, &KEY, [5; 12], &s).unwrap();
    (put(&s, Some(&r)), r)
}

/// A record's sealed bytes as way 3 would hold them.
fn as_way3(r: &Record) -> Sealed {
    let params = Params { m_kib: 0, t: 0, p: 0 };
    Sealed { params, salt: [0; 32], nonce: r.nonce, key_and_tag: r.key_and_tag }
}

#[test]
fn a_record_comes_back_and_a_clear_touches_nothing_else() {
    let (s, r) = prepared();
    assert_eq!(read(&s), Some(r));
    assert_eq!(unwrap(&HAND, &r, &s), Some(KEY));
    let cleared = put(&s, None);
    assert_eq!(read(&cleared), None);
    assert!(cleared[128..196].iter().all(|&b| b == 0));
    assert_eq!((&cleared[..128], &cleared[196..]), (&s[..128], &s[196..]));
    assert!(matches!(parse_key_header(&cleared), Ok(Some(Keyed::Approved(_)))));
}

#[test]
fn a_header_written_alone_holds_no_record() {
    let alone = encode_key_header(&Keyed::Approved(wrap3(&OWN, &KEY, [9; 12]).unwrap()));
    assert_eq!(read(&alone), None);
}

#[test]
fn every_bound_byte_changed_keeps_the_record_shut() {
    let (s, r) = prepared();
    for i in (0..56).chain(128..136) {
        let mut t = s;
        t[i] ^= 1;
        let opened = read(&t).and_then(|r| unwrap(&HAND, &r, &t));
        assert_eq!(opened, None, "byte {i}");
    }
    let mut t = s;
    t[60] ^= 1;
    assert_eq!(unwrap(&HAND, &r, &t), Some(KEY), "way 3's own wrap is not bound");
    assert_eq!(unwrap(&OWN, &r, &s), None, "the own key does not open the record");
}

#[test]
fn a_record_in_way_3s_place_does_not_open_there() {
    let (s, r) = prepared();
    assert_eq!(unwrap3(&HAND, &as_way3(&r)), None);
    let same = wrap(&OWN, &KEY, [5; 12], &s).unwrap();
    assert_eq!(unwrap3(&OWN, &as_way3(&same)), None, "the magic in its associated data");
}
