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

//! Each bound on the header and its metadata, broken once and refused by name.

use super::build::Gguf;
use crate::{parse, GgufError as E, DEFAULT};

fn check(g: &Gguf) -> Result<crate::Summary, E> {
    parse(&mut g.out.as_slice(), g.out.len() as u64, &DEFAULT)
}

#[test]
fn the_header_fields_are_bounded_before_they_size_anything() {
    let mut bad = Gguf::header(3, 0, 0);
    bad.out[0] = b'X';
    assert_eq!(check(&bad), Err(E::BadMagic));
    assert_eq!(check(&Gguf::header(1, 0, 0)), Err(E::UnsupportedVersion(1)));
    assert_eq!(check(&Gguf::header(3, 1 << 40, 0)), Err(E::TooManyTensors(1 << 40)));
    assert_eq!(check(&Gguf::header(3, 0, u64::MAX)), Err(E::TooManyKeys(u64::MAX)));
}

#[test]
fn a_string_or_array_longer_than_its_bound_is_refused_where_it_starts() {
    let mut g = Gguf::header(3, 0, 1);
    g.u64(1 << 40);
    assert_eq!(check(&g), Err(E::StringTooLong { at: 24, len: 1 << 40 }));
    let mut g = Gguf::header(3, 0, 1);
    g.str("k").u32(9).u32(4).u64(1 << 60);
    assert_eq!(check(&g), Err(E::ArrayTooLong { at: 37, len: 1 << 60 }));
    let mut g = Gguf::header(3, 0, 1);
    g.str("k").u32(9).u32(9).u64(1);
    assert_eq!(check(&g), Err(E::NestedArray { at: 37 }));
    let mut g = Gguf::header(3, 0, 1);
    g.str("k").u32(13);
    assert_eq!(check(&g), Err(E::UnknownValueType { at: 33, ty: 13 }));
}

#[test]
fn the_alignment_must_be_a_u32_power_of_two_no_larger_than_a_mebibyte() {
    for a in [0u32, 3, 48, 1 << 21] {
        let mut g = Gguf::header(3, 0, 1);
        g.kv_u32("general.alignment", a);
        assert_eq!(check(&g), Err(E::BadAlignment(a)));
    }
    let mut g = Gguf::header(3, 0, 1);
    g.kv_str("general.alignment", "32");
    assert_eq!(check(&g), Err(E::KeyType { at: 49, ty: 8 }));
}
