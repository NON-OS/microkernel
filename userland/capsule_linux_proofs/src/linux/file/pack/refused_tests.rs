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

//! A run's refused packs said once a reason: the Telegram closure on a
//! machine not let run what it installs gave 253 lines into the Terminal
//! before the one that said why.

use super::refused::lines;

const NOT_OURS: &str = "its index is not this machine's";

#[test]
fn a_closure_refused_for_one_reason_is_one_line() {
    let names: Vec<String> = (0..253).map(|i| format!("/linux-packs/alpine/p{i}.pack")).collect();
    let refused: Vec<(&[u8], &str)> = names.iter().map(|n| (n.as_bytes(), NOT_OURS)).collect();
    assert_eq!(
        lines(&refused),
        ["[LINUX] 253 packs not served: its index is not this machine's (/linux-packs/alpine/p0.pack and 252 more)\n"]
    );
}

#[test]
fn one_pack_is_said_as_before() {
    let refused: [(&[u8], &str); 1] = [(b"/linux-packs/alpine/zlib.pack", "not a pack")];
    assert_eq!(
        lines(&refused),
        ["[LINUX] pack /linux-packs/alpine/zlib.pack not served: not a pack\n"]
    );
}

#[test]
fn each_reason_has_its_line_where_it_first_came() {
    let refused: [(&[u8], &str); 5] = [
        (b"/a.pack", "not a pack"),
        (b"/b.pack", NOT_OURS),
        (b"/c.pack", "not a pack"),
        (b"/d.pack", NOT_OURS),
        (b"/e.pack", "too large"),
    ];
    assert_eq!(
        lines(&refused),
        [
            "[LINUX] 2 packs not served: not a pack (/a.pack and 1 more)\n",
            "[LINUX] 2 packs not served: its index is not this machine's (/b.pack and 1 more)\n",
            "[LINUX] pack /e.pack not served: too large\n",
        ]
    );
}

#[test]
fn nothing_refused_says_nothing_and_a_bad_name_is_kept_readable() {
    assert!(lines(&[]).is_empty());
    let refused: [(&[u8], &str); 1] = [(b"/x\xff.pack", NOT_OURS)];
    assert_eq!(
        lines(&refused),
        ["[LINUX] pack /x\u{fffd}.pack not served: its index is not this machine's\n"]
    );
}
