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

//! Base64 against RFC 4648.

use crate::directory::decode;

// RFC 4648 section 10.
const RFC4648: &[(&str, &[u8])] = &[
    ("", b""),
    ("Zg==", b"f"),
    ("Zm8=", b"fo"),
    ("Zm9v", b"foo"),
    ("Zm9vYg==", b"foob"),
    ("Zm9vYmE=", b"fooba"),
    ("Zm9vYmFy", b"foobar"),
];

#[test]
fn rfc4648_vectors() {
    for (text, want) in RFC4648 {
        assert_eq!(decode(text.as_bytes()).as_deref(), Some(*want), "{text}");
    }
}

#[test]
fn a_symbol_outside_the_alphabet_is_refused() {
    assert_eq!(decode(b"Zm9v*g=="), None);
}

#[test]
fn a_non_canonical_tail_is_refused() {
    assert_eq!(decode(b"Zh=="), None, "the unused bits of 'Zh' are not zero");
    assert_eq!(decode(b"Zm9w"), Some(b"fop".to_vec()), "a canonical tail still decodes");
}
