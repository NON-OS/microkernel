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

//! The file manager's listing pane after a listing failed: it says the files
//! are not available instead of calling the folder empty, and it never keeps
//! the folder it came from on screen under the new folder's name.

use crate::fm_logic::listing_state::{
    empty_listing, keep_after_failure, read_until_silent, status_line, SILENT,
};

#[test]
fn a_failed_listing_is_not_drawn_as_an_empty_folder() {
    let (head, note) = empty_listing(Some("vfs ipc failed"), false);
    assert_eq!(head, "Files are not available");
    assert_eq!(note, "The file store did not answer.");
    assert_ne!(note, empty_listing(None, false).1);
}

#[test]
fn a_failed_listing_wins_over_an_active_filter() {
    assert_eq!(empty_listing(Some("vfs list failed"), true).0, "Files are not available");
}

#[test]
fn each_known_listing_failure_has_its_own_sentence() {
    let notes = ["vfs ipc failed", "vfs list failed", "vfs list malformed", "vfs path invalid"]
        .map(|err| empty_listing(Some(err), false).1);
    for (i, a) in notes.iter().enumerate() {
        assert!(a.ends_with('.'));
        for b in &notes[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn an_unknown_failure_is_shown_as_the_client_worded_it() {
    assert_eq!(empty_listing(Some("access denied"), false).1, "access denied");
}

#[test]
fn an_empty_folder_and_an_empty_filter_keep_their_words() {
    assert_eq!(
        empty_listing(None, false),
        ("Nothing here", "This folder is empty. Press n for a new file, or m for a folder.")
    );
    assert_eq!(empty_listing(None, true), ("Nothing here", "No entry matches the current filter."));
}

#[test]
fn a_refresh_of_the_same_folder_may_keep_its_entries() {
    assert!(keep_after_failure("/docs/", "/docs/", true));
}

#[test]
fn entries_of_the_folder_left_behind_never_stay_under_the_new_one() {
    assert!(!keep_after_failure("/", "/docs/", true));
    assert!(!keep_after_failure("", "/", true));
}

#[test]
fn nothing_listed_means_nothing_to_keep() {
    assert!(!keep_after_failure("/docs/", "/docs/", false));
}

/* The footer said "vfs ipc failed"; it says what the pane says. */
#[test]
fn the_footer_says_the_failure_in_words() {
    assert_eq!(status_line(SILENT), b"The file store did not answer.");
    assert_eq!(status_line("vfs list failed"), b"The file store would not list this folder.");
}

/* Opening the window read three settings files and the folder, each
 * waiting five seconds on a store that did not answer. */
#[test]
fn reading_stops_at_the_first_silence() {
    let mut asked = Vec::new();
    let mut taken = Vec::new();
    let silent = read_until_silent(
        &[1, 2, 3, 4],
        |k| {
            asked.push(k);
            if k == 2 {
                Err(SILENT)
            } else {
                Ok(k * 10)
            }
        },
        |k, v| taken.push((k, v)),
    );
    assert!(silent);
    assert_eq!(asked, [1, 2], "nothing is asked after the silence");
    assert_eq!(taken, [(1, 10)]);
}

#[test]
fn an_answered_error_does_not_stop_the_rest() {
    let mut taken = Vec::new();
    let silent = read_until_silent(
        &[1, 2, 3],
        |k| if k == 2 { Err("not found") } else { Ok(k) },
        |k, v| taken.push((k, v)),
    );
    assert!(!silent);
    assert_eq!(taken, [(1, 1), (3, 3)]);
}
