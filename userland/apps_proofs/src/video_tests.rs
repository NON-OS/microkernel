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

//! The video library when its folders were never read: a scan not yet run or
//! a store that could not list any folder is said as such, never drawn as a
//! library with no videos.

use crate::video_says::{library_unavailable, list_roots, scan_failure, SILENT};

#[test]
fn a_scan_that_listed_no_root_reports_the_first_failure() {
    let results = [Err("vfs ipc failed"), Err("access denied"), Err("vfs ipc failed")];
    assert_eq!(scan_failure(&results), Some("vfs ipc failed"));
}

#[test]
fn one_root_listed_is_a_scan_that_read_the_store() {
    // A missing Clips folder is not a store that could not be read.
    let results = [Ok(()), Err("vfs list failed"), Ok(())];
    assert_eq!(scan_failure(&results), None);
    assert_eq!(scan_failure(&[Ok(()); 5]), None);
}

#[test]
fn an_unreadable_store_is_not_an_empty_library() {
    let silent = library_unavailable(true, Some("vfs ipc failed"));
    assert_eq!(silent, Some(("Videos are not available", "The file store did not answer")));
    let refused = library_unavailable(true, Some("access denied"));
    assert_eq!(
        refused,
        Some(("Videos are not available", "The file store would not list your folders"))
    );
}

#[test]
fn a_scan_not_yet_run_says_it_is_looking() {
    assert_eq!(library_unavailable(false, None).map(|(head, _)| head), Some("Looking for videos"));
}

#[test]
fn a_read_library_with_no_videos_keeps_each_screens_own_words() {
    assert_eq!(library_unavailable(true, None), None);
}

/* Five roots, each a five-second wait on a store that did not answer, and a
 * probe per video after: the first silence ends the scan. */
#[test]
fn listing_stops_at_the_first_silent_root() {
    let mut asked = Vec::new();
    let results: [_; 5] = list_roots(|i| {
        asked.push(i);
        if i == 1 {
            Err(SILENT)
        } else {
            Ok(())
        }
    });
    assert_eq!(asked, [0, 1]);
    assert_eq!(results, [Ok(()), Err(SILENT), Err(SILENT), Err(SILENT), Err(SILENT)]);
    assert_eq!(scan_failure(&results), None, "the root that was listed still counts");
}

#[test]
fn a_store_silent_from_the_start_says_so() {
    let results: [_; 5] = list_roots(|_| Err(SILENT));
    assert_eq!(
        library_unavailable(true, scan_failure(&results)).map(|s| s.1),
        Some("The file store did not answer")
    );
}

#[test]
fn a_refused_root_does_not_stop_the_rest() {
    let mut asked = 0;
    let results: [_; 5] = list_roots(|i| {
        asked += 1;
        if i == 4 {
            Err("vfs list failed")
        } else {
            Ok(())
        }
    });
    assert_eq!(asked, 5);
    assert_eq!(results[4], Err("vfs list failed"));
}
