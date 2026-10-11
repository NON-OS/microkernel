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

//! The fields package authentication reads: a datahash is 64 lowercase hex
//! digits, and base64 is padded only at its end.

use super::tar_build::file;
use crate::install::auth::base64::decode;
use crate::install::auth::pkginfo::datahash;

#[test]
fn a_datahash_is_64_lowercase_hex_digits() {
    let hash =
        |h: &str| datahash(&file(".PKGINFO", format!("pkgname = x\ndatahash = {h}\n").as_bytes()));
    assert_eq!(hash(&"ab".repeat(32)), Some([0xAB; 32]));
    assert_eq!(hash(&"ab".repeat(31)), None);
    assert_eq!(hash(&"AB".repeat(32)), None);
    assert_eq!(hash(&"zz".repeat(32)), None);
}

#[test]
fn base64_padding_comes_only_at_the_end() {
    assert_eq!(decode(b"aGk="), Some(b"hi".to_vec()));
    assert_eq!(decode(b"aGk=aGk="), None);
    assert_eq!(decode(b"a==="), None);
}
