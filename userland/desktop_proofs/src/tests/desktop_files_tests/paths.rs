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

//! The desktop's files are home's, and no name leaves it.

use crate::desktop_home::{home_path, is_entry_name, HOME};

#[test]
fn the_desktop_lists_the_home_directory() {
    assert_eq!(HOME, b"/home/nonos");
}

#[test]
fn a_name_on_the_desktop_is_under_home_not_the_root() {
    assert_eq!(home_path("New Folder").as_deref(), Some("/home/nonos/New Folder"));
    assert_eq!(home_path("notes.txt").as_deref(), Some("/home/nonos/notes.txt"));
    assert_eq!(home_path(".profile").as_deref(), Some("/home/nonos/.profile"));
}

#[test]
fn a_name_that_leaves_home_builds_no_path() {
    for name in ["", ".", "..", "a/b", "/etc", "../etc", "x\0y"] {
        assert!(!is_entry_name(name), "{name:?}");
        assert_eq!(home_path(name), None, "{name:?}");
    }
}
