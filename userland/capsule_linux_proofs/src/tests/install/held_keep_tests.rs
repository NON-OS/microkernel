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

//! An install keeps the store's copy of a dependency a running app holds,
//! and only that: never the package asked for, never one the store lacks,
//! never a refusal that is not "in use".

use alloc::string::String;
use alloc::vec;

use crate::held_keep::keeps;

fn have() -> alloc::vec::Vec<String> {
    vec![String::from("zlib"), String::from("musl")]
}

#[test]
fn a_held_dependency_the_store_has_is_kept() {
    assert!(keeps(true, "zlib", "mpv", &have()));
}

#[test]
fn the_package_asked_for_is_never_kept_in_its_own_place() {
    assert!(!keeps(true, "zlib", "zlib", &have()));
}

#[test]
fn a_package_the_store_lacks_is_not_kept() {
    assert!(!keeps(true, "libass", "mpv", &have()));
}

#[test]
fn only_in_use_is_kept() {
    assert!(!keeps(false, "zlib", "mpv", &have()));
}
