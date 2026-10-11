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

//! Every pinned typehash is recomputed from its type string, so the two can
//! never drift: change a field and this fails until the hash is re-pinned.

mod keccak;

use keccak::keccak;
use nonos_nox_config::{BOND, ESCROW, LISTING, SETTLEMENT};
use nonos_nox_config::{EIP712_DOMAIN_TYPE, EIP712_DOMAIN_TYPEHASH, TYPED, VERSION_HASH};

#[test]
fn every_typehash_is_its_type_string() {
    for t in TYPED {
        assert_eq!(keccak(t.type_string.as_bytes()), t.typehash, "{}", t.type_string);
    }
}

#[test]
fn every_type_string_is_canonical() {
    // EIP-712 hashes the string byte for byte: no spaces after commas, one
    // primary type, no nested struct references to append.
    for t in TYPED {
        let s = t.type_string;
        let open = s.find('(').expect("an opening paren");
        assert!(s.ends_with(')'), "{s}");
        assert!(!s.contains(", ") && !s.contains("\n"), "{s}");
        assert_eq!(s[open + 1..s.len() - 1].split(',').count(), t.fields, "{s}");
        assert_eq!(s.matches('(').count(), 1, "{s}");
    }
}

#[test]
fn the_domain_typehash_and_version_hash() {
    assert_eq!(keccak(EIP712_DOMAIN_TYPE.as_bytes()), EIP712_DOMAIN_TYPEHASH);
    assert_eq!(keccak(b"1"), VERSION_HASH);
}

#[test]
fn every_domain_name_hash() {
    for d in [SETTLEMENT, ESCROW, BOND, LISTING] {
        assert_eq!(keccak(d.name.as_bytes()), d.name_hash, "{}", d.name);
        assert_eq!(d.version, "1");
    }
}

#[test]
fn each_struct_signs_under_the_plan_domain() {
    let names: Vec<&str> = TYPED.iter().map(|t| t.domain.name).collect();
    assert_eq!(names, ["NONOS Settlement", "NONOS Bond", "NONOS Bond"]);
}
