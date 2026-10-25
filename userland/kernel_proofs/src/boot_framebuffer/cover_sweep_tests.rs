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

//! The cover rule against the page-by-page model (L79), over every pair of
//! a framebuffer and one memory BAR the 64-page universe holds at half-page
//! steps. The sweep counts what it met against floors, so it cannot pass by
//! trying nothing: held and not held, near misses a page apart, and pairs
//! held for a page they share with no byte in common.

use super::cover_find::{MMIO, PAGE};
use super::cover_model::{bits, touched, END};
use super::cover_rule::{holds, pages};

/// Every span at half-page steps: `(base, len, its pages as bits)`.
fn spans(least: u64) -> Vec<(u64, u64, u64)> {
    let half = PAGE / 2;
    let starts = (0..END / half).map(|b| b * half);
    let all = starts.flat_map(|b| (least..=(END - b) / half).map(move |l| (b, l * half)));
    all.map(|(b, l)| (b, l, touched(b, l))).collect()
}

#[test]
fn every_pair_in_the_universe_is_held_as_its_pages_say() {
    let (frames, bars) = (spans(1), spans(0));
    let (mut held, mut apart, mut edge, mut page_only) = (0u64, 0u64, 0u64, 0u64);
    for &(fb, len, fm) in &frames {
        assert_eq!(bits(pages(fb, len)), fm, "pages of {fb:#x}+{len:#x}");
        for &(b, size, bm) in &bars {
            let want = fm & bm != 0;
            assert_eq!(holds((fb, len), &[(b, size, MMIO)], 1, MMIO), want, "{b:#x}+{size:#x}");
            let bytes = size > 0 && fb < b + size && b < fb + len;
            held += u64::from(want);
            apart += u64::from(!want);
            edge += u64::from(!want && (fm << 1 | fm >> 1) & bm != 0);
            page_only += u64::from(want && !bytes);
        }
    }
    assert!(held > 40_000_000 && apart > 20_000_000, "held {held}, not held {apart}");
    assert!(edge > 1_000_000 && page_only > 100_000, "edges {edge}, page only {page_only}");
}
