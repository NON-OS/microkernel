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

use capsule_browser_proofs::browser::shell::Shell;
use capsule_browser_proofs::browser::ui::page_part::PagePart;
use nonos_toolkit::widgets::Controls;

use super::fixtures::{bookmarks, tabs};
use super::kit::{at_scale, geom, paint, W};

/* Scrolled `by`, nothing under the header band acts but its own parts. */
fn band_holds(s: &Shell, q: u32) {
    let mut c = Controls::acting_on_press();
    paint(s, &mut c);
    let band = geom(s).band;
    for y in 0..band {
        for x in 0..W {
            let under =
                matches!(c.at(x, y), Some(PagePart::Pill(_) | PagePart::Name | PagePart::Delete));
            assert!(c.at(x, y).is_none() || under, "({x}, {y}) {:?} at {q}", c.at(x, y));
        }
    }
    let first = (band..band + 200).find(|&y| c.at(geom(s).frame.x + 4, y).is_some());
    assert!(first.is_some(), "nothing acts just under the band at {q}");
}

#[test]
fn rows_and_cards_scrolled_under_the_band_take_nothing() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let mut b = bookmarks();
        b.page_scroll = geom(&b).row_h / 2;
        band_holds(&b, q);
        let mut t = tabs(true);
        t.page_scroll = geom(&t).band / 3;
        band_holds(&t, q);
    }
}
