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

use capsule_browser_proofs::browser::event::select_list::MAX_ROWS;
use capsule_browser_proofs::browser::ui::select_menu::{place, row_at};
use nonos_toolkit::widgets::Rect;

use super::kit::{at_scale, inside, open, px, H, PAGE, SIZES, W};
use super::spots::{row_mid, select};

#[test]
fn the_list_opens_under_its_select_or_over_it_inside_the_page() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let s = select();
        let (_, at) = open(SIZES, s, PAGE);
        assert_eq!((at.x, at.y), (s.x, s.y + s.h), "under, from its left edge, at {q}");
        let low = Rect { y: H - px(30), ..s };
        let (_, over) = open(SIZES, low, PAGE);
        assert_eq!(over.y + over.h, low.y, "over, when it does not fit under, at {q}");
        let (_, pulled) = open(SIZES, Rect { x: W - px(40), ..s }, PAGE);
        assert_eq!(pulled.x + pulled.w, W, "pulled in from the right edge at {q}");
        let wide = Rect { w: px(400), ..s };
        assert_eq!(open(SIZES, wide, PAGE).1.w, px(400), "as wide as its select at {q}");
    }
}

#[test]
fn a_long_list_scrolls_inside_the_page_and_keeps_its_width() {
    let long = |i: usize| if i == 30 { "a far longer label than the rest".into() } else { format!("o{i}") };
    let html: String = (0..40).map(|i| format!("<option>{}", long(i))).collect();
    let html = format!("<select>{html}</select>");
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (s, page) = (select(), Rect { h: px(300), ..PAGE });
        let (mut list, at) = open(html.as_bytes(), s, page);
        assert!(list.shown() > 1 && list.shown() < MAX_ROWS, "{} rows at {q}", list.shown());
        assert!(at.y + at.h <= page.h, "inside the page at {q}");
        list.step(35);
        let now = place(&list, s, page);
        assert!(list.first > 0, "scrolled at {q}");
        assert_eq!((now.w, now.h), (at.w, at.h), "the same size scrolled at {q}");
        let hi = row_mid(now, (list.hi - list.first) as u32);
        assert!(inside(now, hi), "the highlight is in view at {q}");
        assert_eq!(row_at(&list, now, hi.0, hi.1), Some(list.hi), "and chosen there at {q}");
    }
}
