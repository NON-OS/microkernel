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

//! The file manager's one-line fields take any printable character, not
//! only ASCII, and hold it whole.

use alloc::string::String;

use crate::fm_logic::text_field::{
    name_char, push_within, query_char, FILTER_MAX, NAME_MAX, QUERY_MAX,
};

#[test]
fn a_name_takes_letters_of_any_script_but_no_white_space() {
    for ch in ['a', 'É', 'ß', 'ж', 'λ', '日', '€', '-', '.', '_'] {
        assert!(name_char(ch), "{ch} belongs in a name");
    }
    for ch in [' ', '\t', '\u{a0}', '\u{3000}', '\u{7}', '\u{85}'] {
        assert!(!name_char(ch), "{ch:?} does not");
    }
    assert!(query_char(' '), "a query may have words");
    assert!(!query_char('\t'));
}

#[test]
fn a_typed_name_is_held_as_utf8_and_backspace_takes_a_character() {
    let mut name = String::new();
    for ch in "Café-日記.txt".chars() {
        assert!(push_within(&mut name, ch, NAME_MAX, name_char));
    }
    assert_eq!(name, "Café-日記.txt");
    name.pop();
    name.pop();
    name.pop();
    name.pop();
    assert_eq!(name, "Café-日記");
    name.pop();
    assert_eq!(name, "Café-日", "the three byte character went whole");
}

#[test]
fn a_full_field_refuses_a_character_that_would_not_fit_whole() {
    let mut f = "a".repeat(FILTER_MAX - 1);
    assert!(!push_within(&mut f, 'é', FILTER_MAX, name_char));
    assert!(push_within(&mut f, 'b', FILTER_MAX, name_char));
    assert!(!push_within(&mut f, 'c', FILTER_MAX, name_char));
    assert_eq!(f.len(), FILTER_MAX);
}

#[test]
fn the_limits_are_the_ones_the_fields_had() {
    // The prompt and search held 64 bytes and the filter 48 before they took
    // anything but ASCII; a name of ASCII fits exactly as it did.
    assert_eq!((NAME_MAX, FILTER_MAX, QUERY_MAX), (64, 48, 64));
    let mut q = String::new();
    for _ in 0..QUERY_MAX {
        assert!(push_within(&mut q, 'x', QUERY_MAX, query_char));
    }
    assert!(!push_within(&mut q, 'x', QUERY_MAX, query_char));
}
