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

//! The markdown viewer's page scrolls on the wheel: three body lines a notch,
//! a notch away from the user (+1) goes up, and the page stops with its last
//! line a margin above the bottom edge.

use crate::mdview_page::layout::{gap, line_height, Line, Style};
use crate::mdview_page::scroll::{max_scroll, place, wheel, wheel_step, TOP};

fn page(n: usize, style: Style) -> Vec<Line> {
    (0..n).map(|_| Line { style, spans: Vec::new(), lead: false }).collect()
}

#[test]
fn lines_sit_where_the_painter_always_put_them() {
    let mut lines = page(3, Style::Body);
    lines[2].lead = true;
    let mut tops = Vec::new();
    let end = place(&lines, |y, _| tops.push(y));
    let h = line_height(Style::Body);
    let g = gap(Style::Body);
    assert_eq!(tops, [TOP, TOP + h, TOP + 2 * h + g], "a lead line after the first leaves a gap");
    assert_eq!(end, TOP + 3 * h + g);
}

#[test]
fn a_notch_is_three_body_lines_toward_the_top_when_away() {
    let lines = page(40, Style::Body);
    let step = wheel_step();
    assert_eq!(step, 3 * line_height(Style::Body) as u32);
    assert_eq!(wheel(0, &lines, 520, -1), step);
    assert_eq!(wheel(2 * step, &lines, 520, 1), step);
}

#[test]
fn the_page_stops_at_its_top_and_with_its_last_line_on_screen() {
    let lines = page(40, Style::Body);
    let end = TOP + 40 * line_height(Style::Body);
    let max = (end + TOP - 520) as u32;
    assert_eq!(max_scroll(&lines, 520), max);
    assert_eq!(wheel(max - 1, &lines, 520, -1), max);
    assert_eq!(wheel(max, &lines, 520, -10), max);
    assert_eq!(wheel(5, &lines, 520, 1), 0);
    assert_eq!(max_scroll(&page(3, Style::Body), 520), 0, "a page that fits does not scroll");
    assert_eq!(wheel(0, &[], 520, -1), 0);
}
