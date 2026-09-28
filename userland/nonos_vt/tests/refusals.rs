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

//! What a program may ask for and does not get, and the ceilings that keep
//! a program from growing the terminal.

#[path = "support/term.rs"]
mod support;
use nonos_vt::limits::MAX_REPLY;
use support::term;

#[test]
fn a_program_that_never_reads_cannot_grow_the_queue() {
    let mut t = term(80, 24, "");
    for _ in 0..10_000 {
        t.feed(b"\x1b[c");
    }
    assert!(t.take_replies().len() <= MAX_REPLY);
}

#[test]
fn clipboard_reads_are_refused_and_writes_are_offered() {
    let mut t = term(80, 24, "\x1b]52;c;?\x07");
    assert!(t.take_replies().is_empty());
    assert_eq!(t.clipboard_reads_refused(), 1);
    t.feed(b"\x1b]52;c;aGk=\x07");
    let r = t.take_clipboard_request().unwrap();
    assert_eq!(r.base64, b"aGk=");
}

#[test]
fn window_manipulation_is_ignored() {
    // Move, resize and iconify: a program may not touch its window.
    let mut t = term(80, 24, "\x1b[3;0;0t\x1b[8;1;1t\x1b[2t");
    assert!(t.take_replies().is_empty());
}

#[test]
fn titles_lose_control_bytes() {
    let t = term(80, 24, "\x1b]0;a\u{9b}b\x07");
    assert_eq!(t.title(), "ab");
}
