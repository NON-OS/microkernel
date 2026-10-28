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

//! Canonical mode as a shell on foot meets it: whole lines, erase and kill,
//! and end of file.

use super::support::{pair, read, type_in, EAGAIN};

#[test]
fn canonical_mode_reads_a_line_only_once_it_ends() {
    let mut p = pair();
    assert_eq!(type_in(&mut p, b"ls"), b"ls", "echoed as typed");
    assert_eq!(read(&mut p), Err(EAGAIN), "no line yet: the reader parks");
    assert_eq!(p.bits(false) & 1, 0, "and poll says not readable");
    // Enter sends CR; ICRNL makes it the newline, ONLCR echoes CR LF.
    assert_eq!(type_in(&mut p, b"\r"), b"\r\n");
    assert_eq!(p.bits(false) & 1, 1);
    assert_eq!(read(&mut p).unwrap(), b"ls\n");
    type_in(&mut p, b"a\nb\n");
    assert_eq!(read(&mut p).unwrap(), b"a\n", "one line per read");
    assert_eq!(read(&mut p).unwrap(), b"b\n");
}

#[test]
fn erase_and_kill_edit_the_line_and_rub_it_out() {
    let mut p = pair();
    assert_eq!(type_in(&mut p, b"abc\x7f"), b"abc\x08 \x08");
    type_in(&mut p, b"\n");
    assert_eq!(read(&mut p).unwrap(), b"ab\n");
    type_in(&mut p, b"junk\x15ok\n");
    assert_eq!(read(&mut p).unwrap(), b"ok\n", "^U dropped the whole line");
    type_in(&mut p, b"one two\x17\n");
    assert_eq!(read(&mut p).unwrap(), b"one \n", "^W dropped the last word");
}

#[test]
fn ctrl_d_ends_the_file_on_an_empty_line_and_sends_a_partial_one() {
    let mut p = pair();
    assert_eq!(type_in(&mut p, b"\x04"), b"", "not echoed");
    assert_eq!(read(&mut p).unwrap(), b"", "end of file");
    assert_eq!(read(&mut p), Err(EAGAIN), "once only");
    type_in(&mut p, b"ab\x04");
    assert_eq!(read(&mut p).unwrap(), b"ab", "the line as it stands, no newline");
}
