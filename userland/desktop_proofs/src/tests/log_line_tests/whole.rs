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

//! A line stays one line, cut and numbered whole.

use nonos_app_skeleton::log_line::{Line, LINE_MAX};

/// A reason with a newline in it (a name, a server's words) would split the
/// line, and the second half would carry no tag `log` could find it by.
#[test]
fn a_newline_in_the_words_does_not_split_the_line() {
    let line = Line::new(b"SHELL").text(b"store: one\ntwo\r\tthree\x7f");
    assert_eq!(line.bytes(), b"[SHELL] store: one two  three \n");
    assert_eq!(line.bytes().iter().filter(|&&b| b == b'\n').count(), 1);
}

/// Cut to what MkDebug takes, the newline kept, never inside a character.
#[test]
fn a_long_line_is_cut_whole() {
    let long = "é".repeat(LINE_MAX);
    let line = Line::new(b"APP-FAIL").text(long.as_bytes()).text(b"never seen");
    let bytes = line.bytes();
    assert!(bytes.len() <= LINE_MAX && bytes.len() <= 256);
    assert_eq!(bytes.last(), Some(&b'\n'));
    assert!(core::str::from_utf8(bytes).is_ok(), "cut inside a character");
    assert!(bytes.starts_with(b"[APP-FAIL] "));
}

#[test]
fn numbers_are_written_whole() {
    assert_eq!(Line::new(b"X").num(0).bytes(), b"[X] 0\n");
    assert_eq!(Line::new(b"X").num(i64::MIN).bytes(), b"[X] -9223372036854775808\n");
    assert_eq!(Line::new(b"X").num(4096).bytes(), b"[X] 4096\n");
}
