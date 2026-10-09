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

//! A save or export that did not land says why in words, and what to do,
//! instead of one line for every failure.

use crate::save_said::{export_failed, save_failed};

const REASONS: [&str; 8] = [
    "vfs ipc failed",
    "no space left",
    "too large",
    "access denied",
    "is a directory",
    "vfs open failed",
    "not found",
    "vfs path invalid",
];

fn text(b: &[u8]) -> &str {
    core::str::from_utf8(b).unwrap()
}

#[test]
fn a_full_store_and_a_silent_one_are_told_apart() {
    assert_eq!(
        text(save_failed("no space left")),
        "save failed: the file store is full; delete files to make room"
    );
    assert_eq!(
        text(save_failed("vfs ipc failed")),
        "save failed: the file store did not answer; try again"
    );
}

#[test]
fn a_missing_folder_says_to_check_it() {
    assert!(text(save_failed("vfs open failed")).ends_with("check the folder exists"));
    assert_eq!(save_failed("not found"), save_failed("vfs open failed"));
}

#[test]
fn each_known_reason_has_its_own_line_and_no_code() {
    let lines: Vec<&str> = REASONS.iter().map(|r| text(save_failed(r))).collect();
    for (i, a) in lines.iter().enumerate() {
        assert!(a.starts_with("save failed: "));
        assert!(!a.contains("vfs") && !a.contains("ipc"), "{a}");
        for (j, b) in lines.iter().enumerate() {
            let same_meaning = (REASONS[i] == "not found" && REASONS[j] == "vfs open failed")
                || (REASONS[j] == "not found" && REASONS[i] == "vfs open failed");
            assert!(i == j || same_meaning || a != b, "{a}");
        }
    }
}

#[test]
fn an_unknown_reason_keeps_the_old_line() {
    assert_eq!(text(save_failed("vfs short write")), "save failed: file could not be written");
    assert_eq!(text(export_failed("vfs short write")), "export failed: file could not be written");
}

#[test]
fn an_export_says_the_same_of_the_export() {
    for r in REASONS {
        let save = text(save_failed(r));
        let export = text(export_failed(r));
        assert!(export.starts_with("export failed: "), "{export}");
        assert!(!export.contains("vfs"), "{export}");
        let tail = |s: &'static str| s.split_once(": ").map(|(_, t)| t).unwrap_or(s);
        assert_eq!(tail(save).split(';').count(), tail(export).split(';').count(), "{r}");
    }
}

/* The status bar holds about 80 characters at its size. */
#[test]
fn every_line_fits_the_status_bar() {
    for r in REASONS {
        assert!(save_failed(r).len() <= 80 && export_failed(r).len() <= 80, "{r}");
    }
}
