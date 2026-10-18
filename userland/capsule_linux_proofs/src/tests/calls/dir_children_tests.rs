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

//! A directory's children come from a listing that matches bytes, so a
//! sibling whose name only begins the same way must not leak in.

use crate::dir_children::children;

fn keys(all: &[&str]) -> Vec<String> {
    all.iter().map(|k| String::from(*k)).collect()
}

#[test]
fn a_sibling_tree_is_not_a_child_of_the_root() {
    let listed = keys(&[
        "/linux/usr/bin/jq",
        "/linux/etc/os-release",
        "/linux-deb/usr/bin/jq",
        "/linux-pacman/usr/bin/nmap",
        "/linux-private/7/tmp/x",
    ]);
    assert_eq!(children(b"/linux", listed), ["usr", "etc"]);
}

#[test]
fn every_file_below_is_cut_to_its_first_name_and_seen_once() {
    let listed = keys(&["/linux/usr/bin/a", "/linux/usr/bin/b", "/linux/usr/lib/c"]);
    assert_eq!(children(b"/linux/usr", listed), ["bin", "lib"]);
}

#[test]
fn a_file_named_like_the_directory_plus_a_suffix_is_not_inside_it() {
    let listed = keys(&["/linux/usr/bin/jq", "/linux/usr/bin/jq-extra/x"]);
    assert_eq!(children(b"/linux/usr/bin/jq", listed), Vec::<String>::new());
}
