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

//! The first key typed after a click into the text goes in, and stays
//! (a boot test finding: "nonos text" typed after a click read
//! "onos text"). A press leaves the anchor on the caret for a drag; an
//! edit must not leave it behind, or the text typed after it reads as
//! selected and the next key types over it.

use crate::edit_tests::{doc, text};

/// A click at `caret` as the editor's press leaves the document: the
/// caret placed there and the anchor on it, ready for a drag.
fn clicked(t: &str, caret: usize) -> crate::state::State {
    let mut s = doc(t);
    s.caret = caret;
    s.anchor_at_caret();
    s
}

fn typed(s: &mut crate::state::State, keys: &str) {
    for ch in keys.chars() {
        let mut b = [0u8; 4];
        s.type_bytes(ch.encode_utf8(&mut b).as_bytes());
    }
}

#[test]
fn every_key_typed_after_a_click_into_an_empty_document_stays() {
    let mut s = clicked("", 0);
    typed(&mut s, "nonos text");
    assert_eq!(text(&s), "nonos text");
    assert_eq!(s.caret, 14);
    assert_eq!(s.sel_range(), None, "nothing typed is selected");
}

#[test]
fn every_key_typed_after_a_click_inside_text_stays() {
    let mut s = clicked("ab", 1);
    typed(&mut s, "xy");
    assert_eq!(text(&s), "axyb");
}

#[test]
fn a_dragged_selection_is_still_typed_over_once() {
    let mut s = clicked("keep DROP keep", 5);
    s.caret = 9;
    typed(&mut s, "new");
    assert_eq!(text(&s), "keep new keep");
}

#[test]
fn backspace_and_enter_after_a_click_leave_no_selection_behind() {
    let mut s = clicked("abc", 3);
    assert!(s.insert_newline());
    typed(&mut s, "d");
    assert_eq!(text(&s), "abc\nd");
    let mut s = clicked("abc", 3);
    assert!(s.backspace());
    typed(&mut s, "x");
    assert_eq!(text(&s), "abx");
}
