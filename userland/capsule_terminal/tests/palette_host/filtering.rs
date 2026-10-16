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

//! Query matching and ranking over the index.

use crate::palette::filter::{filter, matches};
use crate::palette::state::QUERY_CAP;
use crate::support::index;

#[test]
fn an_empty_query_keeps_the_whole_index_in_order() {
    let items = index();
    let mut out = [0usize; 16];
    let n = filter(&items, b"", &mut out);
    assert_eq!(n, items.len());
    assert_eq!(&out[..n], &[0, 1, 2, 3, 4, 5, 6]);
}

#[test]
fn a_query_that_matches_nothing_yields_nothing() {
    let mut out = [0usize; 16];
    assert_eq!(filter(&index(), b"zzzz", &mut out), 0);
}

#[test]
fn ranking_follows_source_order_then_recency() {
    let items = index();
    let mut out = [0usize; 16];
    let n = filter(&items, b"s", &mut out);
    let got: Vec<&str> = out[..n].iter().map(|&i| items[i].label).collect();
    assert_eq!(got, ["history", "git status", "src", "/home/user/src"]);
}

#[test]
fn matching_ignores_case_and_finds_the_middle_of_a_label() {
    assert!(matches("Change Theme", b"theme"));
    assert!(matches("cargo check", b"GO CH"));
    assert!(!matches("ls", b"lsx"));
    assert!(matches("", b""));
    assert!(!matches("", b"a"));
}

#[test]
fn a_label_shorter_than_the_query_is_not_a_match() {
    let long = [b'a'; QUERY_CAP];
    assert!(!matches("a", &long));
}
