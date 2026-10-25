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

//! The cover rule's reach is what the MMIO broker maps (L79). A request for
//! the whole of a BAR maps exactly the pages `pages` gives that BAR; no
//! request inside the BAR maps a page outside them; one reaching past the
//! BAR is refused. So a BAR holds the framebuffer exactly when some request
//! its holder may make maps a page the framebuffer lies in: the rule counts
//! no page a holder cannot reach and misses none it can. The broker's own
//! window rule is the one mmio_window mounts.

use super::cover_byte_gen::{bar, frame};
use super::cover_model::Lcg;
use super::cover_rule::pages;
use crate::mmio_window::window::{window, WindowError};

#[test]
fn a_bars_pages_are_every_page_its_holder_can_map() {
    let mut g = Lcg(0x0079_0EAC_0000_0002);
    let (mut narrower, mut sub_page) = (0u32, 0u32);
    for case in 0..100_000 {
        let fb = frame(&mut g);
        let (b, s) = bar(&mut g, fb);
        let (first, end) = pages(b, s);
        let whole = window(b, s, 0, s, &[]).expect("a whole BAR maps");
        let mapped = (whole.page_start, whole.page_start + whole.page_bytes);
        assert_eq!(mapped, (first, end), "case {case}: {b:#x}+{s:#x}");
        let off = g.below(s);
        let len = 1 + g.below(s - off);
        let part = window(b, s, off, len, &[]).expect("a request inside its BAR maps");
        let (lo, hi) = (part.page_start, part.page_start + part.page_bytes);
        assert!(first <= lo && hi <= end, "case {case}: {b:#x}+{s:#x} at {off:#x}+{len:#x}");
        let past = window(b, s, off, s - off + 1, &[]);
        assert_eq!(past, Err(WindowError::BadRange), "case {case}: past the BAR's end");
        narrower += u32::from(hi - lo < end - first);
        sub_page += u32::from(s < 4096 && end - first == 4096);
    }
    assert!(narrower > 30_000 && sub_page > 18_000, "narrower {narrower}, in a page {sub_page}");
}
