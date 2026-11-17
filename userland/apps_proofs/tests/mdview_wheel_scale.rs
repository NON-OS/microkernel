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

//! The markdown viewer's page scrolls on the wheel at 1x, 1.25x, 1.5x and 2x:
//! three body lines a notch, a notch away from the user (+1) goes up, and the
//! page stops with its last line a margin above the bottom edge. Its own
//! binary, so setting the scale races no other test.

use apps_proofs::mdview_page::layout::{gap, line_height, Line, Style};
use apps_proofs::mdview_page::manifest::HEIGHT;
use apps_proofs::mdview_page::scroll::{max_scroll, place, top, wheel, wheel_step};
use nonos_toolkit::design::scale::{scale, set_scale};

fn page(n: usize, style: Style) -> Vec<Line> {
    (0..n).map(|_| Line { style, spans: Vec::new(), lead: false }).collect()
}

fn lines_sit_where_the_painter_puts_them() {
    let mut lines = page(3, Style::Body);
    lines[2].lead = true;
    let mut tops = Vec::new();
    let end = place(&lines, |y, _| tops.push(y));
    let (t, h, g) = (top(), line_height(Style::Body), gap(Style::Body));
    assert_eq!(tops, [t, t + h, t + 2 * h + g], "a lead line after the first leaves a gap");
    assert_eq!(end, t + 3 * h + g);
}

fn a_notch_is_three_painted_body_lines_toward_the_top_when_away(view: u32) {
    let lines = page(40, Style::Body);
    let mut tops = Vec::new();
    place(&lines, |y, _| tops.push(y));
    let step = wheel_step();
    assert_eq!(step, 3 * line_height(Style::Body) as u32);
    assert_eq!(step, (tops[3] - tops[0]) as u32, "a notch is three lines as painted");
    assert_eq!(wheel(0, &lines, view, -1), step);
    assert_eq!(wheel(2 * step, &lines, view, 1), step);
}

fn the_page_stops_at_its_top_and_with_its_last_line_on_screen(view: u32) {
    let lines = page(40, Style::Body);
    let end = top() + 40 * line_height(Style::Body);
    let max = (end + top() - view as i32) as u32;
    assert_eq!(max_scroll(&lines, view), max);
    assert_eq!(wheel(max - 1, &lines, view, -1), max);
    assert_eq!(wheel(max, &lines, view, -10), max);
    assert_eq!(wheel(5, &lines, view, 1), 0);
    assert_eq!(max_scroll(&page(3, Style::Body), view), 0, "a page that fits does not scroll");
    assert_eq!(wheel(0, &[], view, -1), 0);
}

#[test]
fn the_wheel_scrolls_the_page_alike_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let view = scale().px(HEIGHT);
        lines_sit_where_the_painter_puts_them();
        a_notch_is_three_painted_body_lines_toward_the_top_when_away(view);
        the_page_stops_at_its_top_and_with_its_last_line_on_screen(view);
    }
    set_scale(4);
}
