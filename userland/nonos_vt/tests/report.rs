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

//! Answers a program waits for, and requests that are refused.

#[path = "support/term.rs"]
mod support;

use support::term;

fn replies(input: &str) -> String {
    let mut t = term(80, 24, input);
    String::from_utf8(t.take_replies()).unwrap()
}

#[test]
fn identity_and_version() {
    assert_eq!(replies("\x1b[c"), "\x1b[?62;22c");
    assert_eq!(replies("\x1b[>c"), "\x1b[>1;10;0c");
    assert!(replies("\x1b[>q").starts_with("\x1bP>|NONOS"));
}

#[test]
fn cursor_position_is_one_based() {
    assert_eq!(replies("\x1b[5;10H\x1b[6n"), "\x1b[5;10R");
    assert_eq!(replies("\x1b[5n"), "\x1b[0n");
}

#[test]
fn modes_are_reported() {
    assert_eq!(replies("\x1b[?2004h\x1b[?2004$p"), "\x1b[?2004;1$y");
    assert_eq!(replies("\x1b[?25$p"), "\x1b[?25;1$y");
    assert_eq!(replies("\x1b[?9999$p"), "\x1b[?9999;0$y");
}

#[test]
fn size_is_reported() {
    assert_eq!(replies("\x1b[18t"), "\x1b[8;24;80t");
}

#[test]
fn colours_are_reported() {
    assert_eq!(replies("\x1b]11;?\x07"), "\x1b]11;rgb:0000/0000/0000\x1b\\");
    assert_eq!(replies("\x1b]4;1;#ff0000\x07\x1b]4;1;?\x07"), "\x1b]4;1;rgb:ffff/0000/0000\x1b\\");
}

#[test]
fn terminfo_queries_are_answered() {
    // "TN" and an unknown "zz", hex encoded.
    assert_eq!(
        replies("\x1bP+q544e;7a7a\x1b\\"),
        "\x1bP1+r544E=787465726D2D323536636F6C6F72\x1b\\\x1bP0+r7A7A\x1b\\"
    );
}
