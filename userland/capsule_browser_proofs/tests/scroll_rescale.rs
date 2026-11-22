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

//! A relayout holds the window's scroll to the page it just laid out: at
//! an unchanged scale the offset stays put within range, a scale change
//! carries it through CSS px, and a page that shrank pulls it back to
//! where the page's bottom meets the viewport's.

use capsule_browser_proofs::browser::omnibox::rescale;

const VIEW: u32 = 600;

#[test]
fn same_scale_in_range_is_identity() {
    for s in [0, 1, 250, 1400] {
        assert_eq!(rescale(s, 4, 4, 2000, VIEW), s);
        assert_eq!(rescale(s, 8, 8, 4000, VIEW), s);
    }
}

#[test]
fn doubling_the_scale_doubles_the_offset() {
    assert_eq!(rescale(500, 4, 8, 10_000, VIEW), 1000);
}

#[test]
fn halving_the_scale_is_held_to_the_page() {
    assert_eq!(rescale(2000, 8, 4, 1200, VIEW), 600);
    assert_eq!(rescale(2000, 8, 4, 5000, VIEW), 1000);
}

#[test]
fn a_shrunk_page_at_the_same_scale_clamps() {
    assert_eq!(rescale(3000, 4, 4, 1000, VIEW), 400);
    assert_eq!(rescale(3000, 4, 4, VIEW, VIEW), 0);
}

#[test]
fn an_empty_page_scrolls_nowhere() {
    assert_eq!(rescale(900, 4, 4, 0, VIEW), 0);
    assert_eq!(rescale(900, 4, 8, 0, VIEW), 0);
    assert_eq!(rescale(u32::MAX, 8, 4, 0, VIEW), 0);
}

#[test]
fn a_huge_offset_does_not_overflow() {
    assert_eq!(rescale(u32::MAX, 4, 8, u32::MAX, 0), i32::MAX as u32);
}
