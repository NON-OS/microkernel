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

//! A framebuffer at the edges of a BAR (L79): one page shared is enough,
//! a shared edge is not, and a BAR smaller than a page still reaches the
//! whole page it lies in. The addresses are illustrative.

use super::cover_find::{mine, Rec, KIB, MMIO};
use super::cover_tests::SMALL;

const BASE: u64 = 0x8000_0000;

#[test]
fn a_frame_sharing_only_a_bars_last_page_is_held() {
    let r = Rec::new(&[(BASE, 64 * KIB, MMIO)], false);
    let last = BASE + 64 * KIB - 4 * KIB;
    assert!(mine((last, SMALL), &r), "from the last page's start");
    assert!(mine((last + 0xF00, SMALL), &r), "from the last page's tail");
    // The other way round: a frame whose last page is the BAR's first.
    assert!(mine((BASE - SMALL + 1, SMALL), &r), "its last byte in the BAR");
    // A BAR ending halfway through a page, and a frame starting later in it.
    let half = Rec::new(&[(BASE, 2 * KIB, MMIO)], false);
    assert!(mine((BASE + 3 * KIB, SMALL), &half), "no byte shared, one page");
}

#[test]
fn a_frame_starting_exactly_at_a_bars_end_is_not_held() {
    let r = Rec::new(&[(BASE, 64 * KIB, MMIO)], false);
    assert!(!mine((BASE + 64 * KIB, SMALL), &r), "starting at its end");
    assert!(!mine((BASE - SMALL, SMALL), &r), "ending at its base");
    let half = Rec::new(&[(BASE, 2 * KIB, MMIO)], false);
    assert!(!mine((BASE + 4 * KIB, SMALL), &half), "the page after its last");
}

#[test]
fn a_2k_bar_inside_the_frames_last_page_is_held() {
    // The frame runs 1 KiB into its last page; the BAR is that page's top
    // half, so they share no byte, only the page.
    let fb = (0xC000_0000, SMALL + KIB);
    let end_page = 0xC000_0000 + SMALL;
    let inside = Rec::new(&[(end_page + 2 * KIB, 2 * KIB, MMIO)], false);
    assert!(mine(fb, &inside), "the last page shared");
    let next = Rec::new(&[(end_page + 4 * KIB, 2 * KIB, MMIO)], false);
    assert!(!mine(fb, &next), "the page after the frame's last");
}
