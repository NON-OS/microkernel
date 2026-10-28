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

//! A market release's extras join the closure after the listed package and
//! its needs, each resolved as a need is: once, through the index, counted
//! toward the bound, and a name nothing provides ends it before a fetch.

use super::install_closure_tests::get::closure::{closure, Closed};
use crate::install::index::Index;

/* zathura opens no PDF without its plugin, which nothing depends on. */
const TEXT: &[u8] = b"P:zathura\nV:1\nD:girara\n\nP:girara\nV:1\n\n\
P:zathura-pdf-mupdf\nV:1\nD:zathura mupdf\n\nP:mupdf\nV:1\nD:girara\n\nP:poppler\nV:1\n\n";

fn names(extras: &[&str], max: usize) -> Result<Vec<String>, Closed> {
    let mut index = Index::default();
    index.add(TEXT, 0);
    closure(&index, "zathura", extras, max).map(|p| p.iter().map(|p| p.name.clone()).collect())
}

#[test]
fn each_extra_follows_the_listed_package_and_its_needs() {
    assert_eq!(names(&[], 10).unwrap(), ["zathura", "girara"]);
    let got = names(&["zathura-pdf-mupdf", "poppler"], 10).unwrap();
    assert_eq!(got, ["zathura", "girara", "zathura-pdf-mupdf", "mupdf", "poppler"]);
}

#[test]
fn an_extra_met_already_is_taken_once_and_each_counts_toward_the_bound() {
    assert_eq!(names(&["girara", "zathura"], 10).unwrap(), ["zathura", "girara"]);
    assert_eq!(names(&["zathura-pdf-mupdf"], 3), Err(Closed::TooLarge));
    assert!(names(&["zathura-pdf-mupdf"], 4).is_ok());
}

#[test]
fn an_extra_the_index_lacks_ends_it_before_a_fetch() {
    assert_eq!(names(&["zathura-pdf-poppler"], 10), Err(Closed::NotProvided));
    assert_eq!(names(&["poppler", "ghost"], 10), Err(Closed::NotProvided));
}
