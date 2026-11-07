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

//! No MkData name reaches `/u/` or `/u` itself, and every object path is
//! `/u/<two hex>/<sixteen hex>`, one per number.

use std::collections::BTreeSet;

use super::name_shape::{is_user_root, well_formed};
use super::path::{dir_of, path_of};

#[test]
fn every_object_path_is_under_u_and_made_of_its_number() {
    let mut seen = BTreeSet::new();
    let mut ids: Vec<u64> = (0..5000).collect();
    ids.extend([u64::MAX, u64::MAX - 1, 1 << 63, 0xff, 0x100, 0xdead_beef_0000_00ff]);
    let mut x = 0x9e37_79b9_7f4a_7c15u64;
    for _ in 0..20_000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        ids.push(x);
    }
    ids.sort_unstable();
    ids.dedup();
    for id in ids {
        let p = path_of(id);
        assert_eq!(&p[..3], b"/u/");
        assert_eq!(&p[..5], &dir_of(id));
        assert_eq!(p[5], b'/');
        assert!(p[3..5]
            .iter()
            .chain(&p[6..])
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()));
        assert_eq!(u64::from_str_radix(std::str::from_utf8(&p[6..]).unwrap(), 16).unwrap(), id);
        assert_eq!(
            u8::from_str_radix(std::str::from_utf8(&p[3..5]).unwrap(), 16).unwrap(),
            id as u8
        );
        assert!(seen.insert(p), "two numbers, one path");
    }
}

#[test]
fn no_mkdata_name_reaches_the_objects_or_their_directory() {
    // Every object path, and its directory, is refused by the name shape.
    for id in [0u64, 1, 2, 255, 256, u64::MAX] {
        assert!(!well_formed(&path_of(id)));
        assert!(!well_formed(&dir_of(id)));
    }
    // `/u` alone is well formed, and refused by name.
    assert!(well_formed(b"/u"));
    assert!(is_user_root(b"/u"));
    for other in [&b"/u-x"[..], b"/uu", b"/U", b"/u.", b"/h-home", b"/model.gguf"] {
        assert!(!is_user_root(other), "{other:?}");
    }
    // Any name with a second slash, a dot first, or a byte off the set is not a name.
    let mut x = 7u64;
    let alphabet = b"/u.-_aZ09\0 \xff";
    for _ in 0..200_000 {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let len = (x >> 58) as usize % 12;
        let name: Vec<u8> =
            (0..len).map(|i| alphabet[((x >> (i * 4)) & 0xf) as usize % alphabet.len()]).collect();
        if well_formed(&name) {
            assert!(!name[1..].contains(&b'/'), "{name:?}");
            assert_eq!(name[0], b'/');
            assert_ne!(name[1], b'.');
        }
        if name.starts_with(b"/u/") {
            assert!(!well_formed(&name), "{name:?} reaches under /u/");
        }
    }
}
