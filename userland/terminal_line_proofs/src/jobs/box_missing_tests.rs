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

//! A report that never came whole is said on the screen, with why when the
//! capsule failed, and on the serial log as a `[BOX-MISSING]` line with
//! every field, sent wherever the glue says one is missing.

use super::box_missing::{line, said, Missing, PANIC_EXIT};

#[test]
fn a_capsule_panic_is_said_to_be_one() {
    let s = String::from_utf8(said(Missing::None, PANIC_EXIT).to_vec()).unwrap();
    assert_eq!(
        s,
        "linux: the box sent no report: capsule_linux stopped on an internal error (exit 134), \
         so what the program tried is not known"
    );
    let l = line(Missing::None, PANIC_EXIT);
    assert_eq!(l, "[BOX-MISSING] v=1 report=none capsule_failed=yes exit=134 source=terminal");
}

#[test]
fn every_other_missing_report_keeps_its_words() {
    let none = b"linux: the box sent no report; the run ended before it could";
    assert_eq!(said(Missing::None, 1), none);
    assert_eq!(said(Missing::None, 0), none);
    assert_eq!(said(Missing::Incomplete, 134), b"linux: the box's report arrived incomplete");
    assert_eq!(said(Missing::Stopped, 130), b"linux: stopped before the box's report arrived");
    let l = |m, e| line(m, e);
    assert_eq!(
        l(Missing::None, 1),
        "[BOX-MISSING] v=1 report=none capsule_failed=no exit=1 source=terminal"
    );
    assert_eq!(
        l(Missing::Incomplete, 134),
        "[BOX-MISSING] v=1 report=incomplete capsule_failed=no exit=134 source=terminal"
    );
    assert_eq!(
        l(Missing::Stopped, 130),
        "[BOX-MISSING] v=1 report=stopped capsule_failed=no exit=130 source=terminal"
    );
}

#[test]
fn the_line_is_never_a_box_line() {
    for m in [Missing::None, Missing::Incomplete, Missing::Stopped] {
        assert!(!line(m, 0).starts_with("[BOX] "), "a reader of [BOX] lines never takes it");
    }
}

#[test]
fn every_missing_report_is_logged_by_the_glue() {
    let draw = include_str!("../../../capsule_terminal/src/jobs/box_draw.rs");
    let body = &draw[draw.find("pub(super) fn missing(").unwrap()..];
    let said_at = body.find("out.writeln_error(said(m, exit));").unwrap();
    assert!(said_at < body.find("to_log(l.as_bytes())").unwrap());
    assert!(draw.contains("let exit = hold.ended.unwrap_or(0);"));
}
