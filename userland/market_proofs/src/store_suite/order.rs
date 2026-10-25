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

//! The order the marketplace window asks the market in.

use alloc::vec::Vec;

use super::fixture::{described, listing};
use crate::store::fill_order::next;
use crate::store::listing::Listing;

#[test]
fn the_selection_is_asked_about_first() {
    let all: Vec<Listing> = vec![listing(b"a", b"A"), listing(b"b", b"B")];
    assert_eq!(next(&all, Some(1), 0), Some((1, true)));
    assert_eq!(next(&all, None, 0), Some((0, false)));
}

#[test]
fn a_selection_already_judged_is_not_asked_again() {
    let mut all: Vec<Listing> = vec![listing(b"a", b"A"), described(listing(b"b", b"B"), b"d")];
    all[1].known.judged = true;
    assert_eq!(next(&all, Some(1), 0), Some((0, false)));
}

#[test]
fn every_description_is_asked_once_and_then_nothing() {
    let mut all: Vec<Listing> = vec![listing(b"a", b"A"), listing(b"b", b"B")];
    let mut asked = 0;
    while let Some((at, selected)) = next(&all, None, 0) {
        assert!(!selected);
        all[at].known.described = true;
        asked += 1;
        assert!(asked <= all.len(), "asked again");
    }
    assert_eq!(asked, 2);
}

#[test]
fn a_selection_past_the_list_asks_about_nothing_but_descriptions() {
    let all: Vec<Listing> = vec![described(listing(b"a", b"A"), b"d")];
    assert_eq!(next(&all, Some(5), 0), None);
}
