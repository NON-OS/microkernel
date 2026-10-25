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

//! The hashes a file compiles to are capsule_linux's: the presets as Track B
//! pinned them, an `allow:` box's whatever order its folders are written in.

use super::{folders, hash_of, word_hash};

const STRICT: &str = "5811b081f826f154ce1ac5a7b558bff858ae0bacc27ad2db9d2e5f84ce05323a";
const OFFLINE: &str = "630d02b9ab8f611ff1d39ba26c29be18566995b5663fd25c0965c47cb4a66776";

#[test]
fn the_presets_hash_as_pinned() {
    assert_eq!(hash_of(&["nonos-box 1", "base strict", "network none"]), STRICT);
    assert_eq!(hash_of(&["nonos-box 1", "base offline", "network none"]), OFFLINE);
}

#[test]
fn a_folders_file_hashes_as_its_allow_word() {
    let h = folders(&["/tmp/b", "/home/a"]);
    assert_eq!(h, word_hash(b"allow:/home/a,/tmp/b"));
    assert_eq!(h, word_hash(b"allow:/tmp/b,/home/a"));
}

#[test]
fn line_order_comments_and_spaces_change_nothing() {
    let a = folders(&["/tmp/a", "/tmp/b"]);
    let b = hash_of(&[
        "# a comment before the header",
        "nonos-box   1",
        "",
        "   network none   ",
        "write /tmp/b",
        "# between",
        "write /tmp/a",
        "base folders",
    ]);
    assert_eq!(a, b);
}

#[test]
fn every_difference_changes_the_hash() {
    let all = [
        hash_of(&["nonos-box 1", "base strict", "network none"]),
        hash_of(&["nonos-box 1", "base offline", "network none"]),
        folders(&["/tmp"]),
        folders(&["/tmp/a"]),
        folders(&["/tmp/a", "/tmp/b"]),
        folders(&["/home/a"]),
    ];
    for (i, a) in all.iter().enumerate() {
        assert!(all[i + 1..].iter().all(|b| b != a), "box {i} shares a hash");
    }
}
