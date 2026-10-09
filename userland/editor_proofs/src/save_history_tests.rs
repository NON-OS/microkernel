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

//! The save point across dropped history and a reload.

use crate::edit_tests::doc;

#[test]
fn an_edit_after_undoing_past_the_save_never_matches_it_again() {
    let mut s = doc("abc");
    s.caret = 3;
    assert!(s.insert(b"X"));
    s.mark_saved();
    assert!(s.undo());
    // A different edit replaces the redo branch that held the saved text.
    assert!(s.insert(b"Y"));
    assert!(s.dirty);
    assert!(s.undo());
    assert!(s.dirty, "abc is not the saved abcX");
}

#[test]
fn dropped_history_cannot_undo_to_a_false_clean() {
    let mut s = doc("");
    // Separate steps: a space after each letter breaks the typing run.
    for _ in 0..600 {
        assert!(s.apply_edit(s.len, 0, b"ab"));
    }
    while s.undo() {}
    assert!(s.len > 0, "the oldest steps were dropped");
    assert!(s.dirty, "the buffer is not the empty text that was loaded");
}

#[test]
fn a_reload_forgets_the_old_history() {
    let mut s = doc("abc");
    s.caret = 3;
    assert!(s.insert(b"d"));
    s.reset_history();
    assert!(!s.dirty);
    assert!(!s.undo(), "undo must not replay edits made to another text");
    assert!(s.sel_anchor.is_none());
}
