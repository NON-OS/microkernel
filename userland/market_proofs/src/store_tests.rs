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

//! The marketplace window's search, and the order it asks the market in.

use alloc::vec::Vec;

use nonos_market_proto::App;

use crate::store::fill_order::next;
use crate::store::listing::Listing;
use crate::store::search::Search;

fn typed(text: &[u8]) -> Search {
    let mut s = Search::default();
    for b in text {
        assert!(s.push(*b));
    }
    s
}

fn listing(id: &[u8], name: &[u8]) -> Listing {
    Listing::new(id.to_vec(), [0; 32], name.to_vec(), true)
}

fn described(mut l: Listing, description: &[u8]) -> Listing {
    let detail = App {
        name: l.name.clone(),
        publisher: b"p".to_vec(),
        description: description.to_vec(),
        releases: 1,
    };
    l.known.detail = Some(detail);
    l.known.described = true;
    l
}

#[test]
fn an_empty_query_keeps_everything() {
    assert!(Search::default().matches(b"htop", None));
    assert!(Search::default().matches(b"", None));
}

#[test]
fn the_name_matches_in_any_case() {
    let s = typed(b"HTo");
    assert!(s.matches(b"htop", None));
    assert!(s.matches(b"My HTOP", None));
    assert!(!s.matches(b"top", None));
}

#[test]
fn a_description_matches_once_it_is_known() {
    let s = typed(b"viewer");
    assert!(!s.matches(b"htop", None));
    assert!(s.matches(b"htop", Some(b"An interactive process Viewer")));
    assert!(!s.matches(b"htop", Some(b"a monitor")));
}

#[test]
fn a_query_longer_than_the_name_matches_nothing() {
    assert!(!typed(b"htopx").matches(b"htop", None));
}

#[test]
fn the_field_holds_at_most_sixty_four_bytes() {
    let mut s = typed(&[b'a'; 64]);
    assert!(!s.push(b'a'));
    assert!(s.pop());
    s.close();
    assert!(s.text().is_empty() && !s.active);
}

#[test]
fn the_selection_is_asked_about_first() {
    let all: Vec<Listing> = vec![listing(b"a", b"A"), listing(b"b", b"B")];
    assert_eq!(next(&all, Some(1)), Some((1, true)));
    assert_eq!(next(&all, None), Some((0, false)));
}

#[test]
fn a_selection_already_judged_is_not_asked_again() {
    let mut all: Vec<Listing> = vec![listing(b"a", b"A"), described(listing(b"b", b"B"), b"d")];
    all[1].known.judged = true;
    assert_eq!(next(&all, Some(1)), Some((0, false)));
}

#[test]
fn every_description_is_asked_once_and_then_nothing() {
    let mut all: Vec<Listing> = vec![listing(b"a", b"A"), listing(b"b", b"B")];
    let mut asked = 0;
    while let Some((at, selected)) = next(&all, None) {
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
    assert_eq!(next(&all, Some(5)), None);
}
