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

//! A pasted line into the file manager's name prompt, filter and search box.

use alloc::string::String;

use crate::fm_logic::text_field::{
    name_char, paste_within, query_char, FieldPaste, NAME_MAX, QUERY_MAX,
};

#[test]
fn a_copied_name_goes_in_whole() {
    let mut name = String::from("draft-");
    assert_eq!(paste_within(&mut name, "Résumé_2026.pdf", NAME_MAX, name_char), FieldPaste::Whole);
    assert_eq!(name, "draft-Résumé_2026.pdf");
}

#[test]
fn a_name_with_a_space_is_refused_whole() {
    let mut name = String::from("x");
    for bad in ["my file.txt", "a\tb", "nbsp\u{a0}name"] {
        assert_eq!(paste_within(&mut name, bad, NAME_MAX, name_char), FieldPaste::Refused);
        assert_eq!(name, "x", "nothing of {bad:?} went in");
    }
}

#[test]
fn a_search_takes_words_and_reads_a_tab_as_a_space() {
    let mut q = String::new();
    assert_eq!(paste_within(&mut q, "tax\treturn 2026", QUERY_MAX, query_char), FieldPaste::Whole);
    assert_eq!(q, "tax return 2026");
}

#[test]
fn control_characters_are_dropped_not_pasted() {
    let mut q = String::new();
    assert_eq!(paste_within(&mut q, "\u{1b}[1mbold", QUERY_MAX, query_char), FieldPaste::Whole);
    assert_eq!(q, "[1mbold");
    let mut q = String::new();
    assert_eq!(paste_within(&mut q, "\u{7}\u{8}", QUERY_MAX, query_char), FieldPaste::Empty);
    assert_eq!(paste_within(&mut q, "", QUERY_MAX, query_char), FieldPaste::Empty);
}

#[test]
fn a_long_name_is_cut_on_a_character() {
    let mut name = "a".repeat(NAME_MAX - 1);
    assert_eq!(paste_within(&mut name, "日本", NAME_MAX, name_char), FieldPaste::Cut);
    assert_eq!(name.len(), NAME_MAX - 1, "a three byte character does not fit in one");
    let mut name = String::new();
    assert_eq!(paste_within(&mut name, &"é".repeat(40), NAME_MAX, name_char), FieldPaste::Cut);
    assert_eq!(name.chars().count(), 32);
}
