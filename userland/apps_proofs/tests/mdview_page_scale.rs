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

//! The markdown viewer keeps logical metrics and draws them at the display
//! scale: at 1.25x, 1.5x and 2x every size is its 1x size through
//! `scale().px` or `scale().font`, and in a window grown by the same scale
//! the column keeps its room and the last line still lands on screen. Its own
//! binary, so setting the scale races no other test.

use apps_proofs::mdview_page::draw::margin;
use apps_proofs::mdview_page::layout::{gap, indent, line_height, px, Line, Style};
use apps_proofs::mdview_page::manifest::{HEIGHT, WIDTH};
use apps_proofs::mdview_page::scroll::{max_scroll, place, top};
use nonos_toolkit::design::scale::{scale, set_scale};

const STYLES: [Style; 6] =
    [Style::H1, Style::H2, Style::H3, Style::Body, Style::Bullet, Style::Code];

fn sizes() -> Vec<(f32, i32, i32, i32)> {
    STYLES.iter().map(|&s| (px(s), line_height(s), gap(s), indent(s))).collect()
}

fn mixed_page() -> Vec<Line> {
    let line = |i: usize| Line { style: STYLES[i % 6], spans: Vec::new(), lead: i % 2 == 0 };
    (0..60).map(line).collect()
}

#[test]
fn every_size_grows_with_the_scale_and_the_page_still_fits() {
    set_scale(4);
    let (at_1x, edges_1x) = (sizes(), (margin(), top()));
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let s = scale();
        let grown = |n: i32| s.px(n as u32) as i32;
        let want: Vec<_> =
            at_1x.iter().map(|&(f, h, g, i)| (s.font(f), grown(h), grown(g), grown(i))).collect();
        assert_eq!(sizes(), want, "q={q}: a size is not its 1x size at the scale");
        assert_eq!((margin(), top()), (grown(edges_1x.0), grown(edges_1x.1)), "q={q}");
        let (w, h) = (s.px(WIDTH) as i32, s.px(HEIGHT) as i32);
        let deepest = STYLES.iter().map(|&st| indent(st)).max().unwrap_or(0);
        assert!(margin() + deepest < w - margin(), "q={q}: no room left for text");
        let lines = mixed_page();
        let max = max_scroll(&lines, h as u32) as i32;
        assert!(max > 0, "q={q}: the page must scroll for this to prove anything");
        let mut last = (0, 0);
        place(&lines, |y, l| last = (y - max, line_height(l.style)));
        assert!(last.0 >= 0 && last.0 + last.1 <= h, "q={q}: last line off screen {last:?}");
        assert_eq!(last.0 + last.1 + top(), h, "q={q}: the page stops a margin above the edge");
    }
    set_scale(4);
}
