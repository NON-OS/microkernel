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

//! The marketplace window's search (the order it asks in: store_order_tests).

use super::store_suite::fixture::typed;
use crate::store::search::Search;

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
