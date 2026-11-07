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

//! Lines come out whole and in order however the input is cut; a line over
//! the cap is dropped to its newline and reported once; nothing is held
//! past the cap.

use crate::line::{Line, Lines, MAX_LINE};

fn cmd(s: &[u8]) -> Line {
    Line::Command(s.to_vec())
}

#[test]
fn whole_lines_come_out_in_order() {
    let mut l = Lines::default();
    assert_eq!(l.feed(b"echo a\npwd\n"), [cmd(b"echo a"), cmd(b"pwd")]);
    assert_eq!(l.feed(b""), []);
    assert_eq!(l.feed(b"\n"), [cmd(b"")]);
}

#[test]
fn a_line_cut_anywhere_is_the_same_line() {
    let input = b"echo nsh-$((6*7))\nexit\n";
    for cut in 0..=input.len() {
        let mut l = Lines::default();
        let mut got = l.feed(&input[..cut]);
        got.extend(l.feed(&input[cut..]));
        assert_eq!(got, [cmd(b"echo nsh-$((6*7))"), cmd(b"exit")], "cut at {cut}");
    }
}

#[test]
fn a_line_at_the_cap_is_kept() {
    let mut l = Lines::default();
    let mut input = vec![b'x'; MAX_LINE];
    input.push(b'\n');
    assert_eq!(l.feed(&input), [cmd(&input[..MAX_LINE])]);
}

#[test]
fn a_line_over_the_cap_is_dropped_to_its_newline_and_reported_once() {
    let mut l = Lines::default();
    assert_eq!(l.feed(&vec![b'x'; MAX_LINE]), []);
    assert_eq!(l.feed(b"y"), [Line::TooLong]);
    for _ in 0..100 {
        assert_eq!(l.feed(&vec![b'z'; 65_536]), [], "dropped, not reported again");
    }
    assert_eq!(l.feed(b"tail\npwd\n"), [cmd(b"pwd")], "the dropped line ends at its newline");
}

#[test]
fn input_with_no_newline_is_never_held_past_the_cap() {
    let mut l = Lines::default();
    let mut reports = 0;
    for _ in 0..1000 {
        reports += l.feed(&[b'a'; 1500]).len();
    }
    assert_eq!(reports, 1);
    assert_eq!(l.feed(b"\nok\n"), [cmd(b"ok")]);
}
