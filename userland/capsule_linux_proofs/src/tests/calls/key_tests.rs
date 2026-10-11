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


//! The store key a guest path becomes, and the root it cannot leave.

use crate::resolve::{key, visible};

fn stored(cwd: &str, path: &str) -> String {
    String::from_utf8(key(&visible(cwd.as_bytes(), path.as_bytes())).as_bytes().to_vec()).unwrap()
}

#[test]
fn every_key_sits_under_the_linux_root() {
    assert_eq!(stored("/", "/etc/passwd"), "/linux/etc/passwd");
    assert_eq!(stored("/home", "lib"), "/linux/home/lib");
}

#[test]
fn the_guest_root_is_the_store_root_itself() {
    assert_eq!(stored("/", "/"), "/linux");
    assert_eq!(stored("/", ""), "/linux");
}

#[test]
fn dot_dot_cannot_climb_out_of_the_linux_root() {
    assert_eq!(stored("/", "../../system/keys"), "/linux/system/keys");
    assert_eq!(stored("/a", "../../../.."), "/linux");
    assert!(stored("/", "/../..//../x").starts_with("/linux/"));
}
