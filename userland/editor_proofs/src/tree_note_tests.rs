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

//! The explorer says its last failure in words, whether or not the tree has
//! rows, and says nothing when nothing failed.

use crate::tree_note::{empty_tree_line, explorer_note};

#[test]
fn nothing_failed_means_no_note() {
    assert_eq!(explorer_note(""), None);
    assert_eq!(empty_tree_line(""), "(empty)");
}

#[test]
fn a_store_that_did_not_answer_is_said_in_words() {
    assert_eq!(explorer_note("vfs ipc failed"), Some("store did not answer"));
    assert_eq!(empty_tree_line("vfs ipc failed"), "store did not answer");
}

#[test]
fn a_refused_delete_or_rename_keeps_the_stores_reason() {
    for reason in ["directory not empty", "access denied", "already exists", "not found"] {
        assert_eq!(explorer_note(reason), Some(reason));
    }
}

#[test]
fn every_note_fits_the_header_band() {
    // The band right of EXPLORER holds about 18 characters at 11 px.
    for status in ["vfs ipc failed", "vfs list failed", "vfs path invalid", "vfs open failed"] {
        let note = explorer_note(status).expect("a note");
        assert!(note.len() <= 26, "{note}");
    }
}
