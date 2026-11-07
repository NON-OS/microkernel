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

//! What a new passphrase must be.

use nonos_lock::rules::{acceptable, checkable, chosen, Refusal, MAX_BYTES, MIN_CHARS};

#[test]
fn length_is_counted_in_characters_and_capped_in_bytes() {
    assert_eq!(MIN_CHARS, 8);
    assert_eq!(acceptable(b"seven c"), Err(Refusal::Short));
    assert_eq!(acceptable(b"eight ch"), Ok(()));
    // Eight characters of two bytes each.
    assert_eq!(acceptable("éééééééé".as_bytes()), Ok(()));
    assert_eq!(acceptable("ééééééé".as_bytes()), Err(Refusal::Short));
    assert_eq!(acceptable(&[b'a'; MAX_BYTES]), Ok(()));
    assert_eq!(acceptable(&[b'a'; MAX_BYTES + 1]), Err(Refusal::Long));
}

#[test]
fn spaces_alone_and_bytes_that_are_not_text_are_refused() {
    assert_eq!(acceptable(b"          "), Err(Refusal::Short));
    assert_eq!(acceptable(&[0xFF; 12]), Err(Refusal::NotText));
}

#[test]
fn the_confirmation_must_match() {
    assert_eq!(chosen(b"long enough", b"long enough"), Ok(()));
    assert_eq!(chosen(b"long enough", b"long enougH"), Err(Refusal::Mismatch));
    assert_eq!(chosen(b"short", b"short"), Err(Refusal::Short));
}

#[test]
fn anything_typed_that_fits_may_be_checked() {
    assert!(!checkable(b""));
    assert!(checkable(b"x"));
    assert!(checkable(&[b'x'; MAX_BYTES]));
    assert!(!checkable(&[b'x'; MAX_BYTES + 1]));
}

#[test]
fn a_typed_field_takes_characters_whole_and_wipes_on_clear() {
    use nonos_lock::field::{Field, FIELD_MAX};
    let mut f = Field::default();
    for c in "pässwörd".chars() {
        assert!(f.push(c));
    }
    assert_eq!(f.chars(), 8);
    assert_eq!(f.bytes(), "pässwörd".as_bytes());
    f.pop();
    f.pop();
    assert_eq!(f.bytes(), "pässwö".as_bytes(), "a two-byte character goes whole");
    f.clear();
    assert!(f.is_empty());
    for _ in 0..FIELD_MAX {
        assert!(f.push('x'));
    }
    assert!(!f.push('x'));
}
