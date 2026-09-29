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

//! Proofs for the save point: the document is clean exactly when its text is
//! the text last read or written, whatever undo and redo did in between.

use crate::edit_tests::{doc, text};

#[test]
fn a_loaded_document_is_clean() {
    let s = doc("abc");
    assert!(!s.dirty);
}

#[test]
fn undo_back_to_the_saved_text_is_clean() {
    let mut s = doc("abc");
    s.caret = 3;
    assert!(s.insert(b"d"));
    assert!(s.dirty);
    assert!(s.undo());
    assert_eq!(text(&s), "abc");
    assert!(!s.dirty, "the text on disk is back");
}

#[test]
fn undo_after_a_save_is_dirty() {
    let mut s = doc("abc");
    s.caret = 3;
    assert!(s.insert(b"d"));
    s.mark_saved();
    assert!(!s.dirty);
    assert!(s.undo());
    assert!(s.dirty, "the buffer no longer matches the saved file");
    assert!(s.redo());
    assert!(!s.dirty, "redo returns to the saved text");
}

#[test]
fn typing_on_after_a_mid_word_save_is_dirty() {
    let mut s = doc("");
    for b in b"ab" {
        assert!(s.insert(&[*b]));
    }
    s.mark_saved();
    // The next letter merges into the same undo step; it is still a change.
    assert!(s.insert(b"c"));
    assert!(s.dirty);
}
