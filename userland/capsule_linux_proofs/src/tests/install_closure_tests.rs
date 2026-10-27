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

//! Fast downloads, D: an install's closure is known whole before anything
//! is fetched, in the order it has always been placed (install/get/
//! closure.rs).

use crate::install::index::{self, Index};

#[path = "../../../capsule_linux/src/linux/install/get"]
mod get {
    pub mod closure;
}
use get::closure::{closure, Closed};

const TEXT: &[u8] = b"P:app\nV:1\nS:1000\nD:liba so:libb.so.1\n\nP:liba\nV:1\nD:libc\n\n\
P:libb\nV:1\np:so:libb.so.1=1\nD:libc\n\nP:libc\nV:1\n\n";

fn names(index: &Index, name: &str, max: usize) -> Result<Vec<String>, Closed> {
    closure(index, name, max).map(|pkgs| pkgs.iter().map(|p| p.name.clone()).collect())
}

#[test]
fn the_listed_package_first_then_each_need_as_popped_once() {
    let mut index = Index::default();
    index.add(TEXT, 0);
    assert_eq!(names(&index, "app", 10).unwrap(), ["app", "libb", "libc", "liba"]);
    assert_eq!(index.by_name("app").map(|p: &index::Pkg| p.size), Some(1000));
}

#[test]
fn a_need_nothing_provides_or_too_many_ends_it_before_a_fetch() {
    let mut index = Index::default();
    index.add(b"P:app\nV:1\nD:ghost\n\n", 0);
    assert_eq!(names(&index, "app", 10), Err(Closed::NotProvided));
    index.add(TEXT, 0);
    assert_eq!(names(&index, "missing", 10), Err(Closed::NotProvided));
    let mut index = Index::default();
    index.add(TEXT, 0);
    assert_eq!(names(&index, "app", 3), Err(Closed::TooLarge));
    assert!(names(&index, "app", 4).is_ok());
}
