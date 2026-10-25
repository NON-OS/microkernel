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

//! The refusals of what a file names and of the file as a whole.

use agentproof_policy::format::{Error, MAX_FILE};
use agentproof_policy::parse;

use super::refusal_tests::refused;

#[test]
fn a_folder_is_one_the_box_can_keep() {
    let h = "nonos-box 1";
    let w = |f: &'static str| refused(&[h, "base folders", f]);
    assert_eq!(w("write tmp/a"), (3, "a folder is named from /"));
    assert_eq!(w("write /tmp/a/"), (3, "a folder has no empty, . or .. part"));
    assert_eq!(w("write /tmp/../etc"), (3, "a folder has no empty, . or .. part"));
    assert_eq!(w("write /tmp//a"), (3, "a folder has no empty, . or .. part"));
    let inside = "a folder is inside /tmp, /home, /root, /run, /var/tmp or /dev/shm";
    assert_eq!(w("write /etc"), (3, inside));
    assert_eq!(w("write /tmpx"), (3, inside));
    assert_eq!(refused(&[h, "write /tmp/a", "write /tmp/a"]), (3, "a folder is named once"));
    let many: Vec<String> = (0..17).map(|i| format!("write /tmp/{i}")).collect();
    let mut lines = vec![h, "base folders"];
    lines.extend(many.iter().map(String::as_str));
    assert_eq!(refused(&lines), (19, "a file has at most 16 write lines"));
}

#[test]
fn the_whole_file_says_enough_and_no_more() {
    let h = "nonos-box 1";
    assert_eq!(refused(&[h, "network none"]), (0, "base is missing"));
    assert_eq!(refused(&[h, "base strict"]), (0, "network is missing; write `network none`"));
    let none = "base folders needs at least one write line";
    assert_eq!(refused(&[h, "base folders", "network none"]), (0, none));
    let own = "write lines go with base folders; strict and offline name their own";
    assert_eq!(refused(&[h, "write /tmp/a", "base offline", "network none"]), (2, own));
    assert_eq!(refused(&[h, "base strict", "network none", "write /tmp/a"]), (4, own));
}

#[test]
fn only_plain_ascii_and_a_bounded_size() {
    let line = "a line is printable ASCII and spaces, with no tab or carriage return";
    let bad: [(&[u8], usize); 4] = [
        (b"nonos-box 1\r\n", 1),
        (b"nonos-box\t1\n", 1),
        (b"nonos-box 1\nbase \xc3\xa9\n", 2),
        (b"\0", 1),
    ];
    for (text, at) in bad {
        assert_eq!(parse(text).unwrap_err(), Error { line: at, why: line });
    }
    let big = vec![b'#'; MAX_FILE + 1];
    assert_eq!(parse(&big).unwrap_err().why, "a policy file is at most 16384 bytes");
    assert!(parse(&vec![b'#'; MAX_FILE]).is_err_and(|e| e.line == 0));
}
