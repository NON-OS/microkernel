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

//! Menu: a tag leads a row's label. The runs each row draws, as recorded,
//! so the tag column and the labels can be read back per row.

use nonos_toolkit::icons::IconId;
use nonos_toolkit::paint::measure_ttf;
use nonos_toolkit::widgets::{Item, Menu, Rect, Surface};

use super::samples::LONG;

/// Tags of unlike widths and scripts, and a set with one too long for any
/// menu.
pub(crate) const TAG_SETS: [&[&str]; 2] = [&["cmd", "run", "日本", "Ü"], &[LONG, "a"]];

/// `items` with item `i` tagged `tags[i % tags.len()]`.
pub(crate) fn tagged(items: &[Item<'static>], tags: &[&'static str]) -> Vec<Item<'static>> {
    items.iter().enumerate().map(|(i, it)| it.tag(tags[i % tags.len()])).collect()
}

/// One run of text: its line top, its left and right edges, and its size.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Run {
    pub(crate) top: i32,
    pub(crate) x: i32,
    pub(crate) end: i32,
    pub(crate) px: f32,
}

/// A surface that keeps the runs of text drawn on it and paints nothing.
struct Spans(Vec<Run>);

impl Surface for Spans {
    fn fill_rect(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn blend_rect(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn fill_round(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn stroke_round(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn circle(&mut self, _: u32, _: u32, _: u32, _: u32) {}
    fn text_ttf(&mut self, x: i32, top: i32, text: &str, _: u32, px: f32) -> i32 {
        let end = x + measure_ttf(text, px);
        self.0.push(Run { top, x, end, px });
        end
    }
    fn icon(&mut self, _: IconId, _: u32, _: u32, _: u32, _: u32) {}
}

/// The runs `m` draws in `at`, one list per row in the order drawn: the
/// tag, if the row has one, then the label, then the shortcut.
pub(crate) fn rows(m: Menu, at: Rect) -> Vec<Vec<Run>> {
    let mut spans = Spans(Vec::new());
    m.draw(&mut spans, at);
    let mut rows: Vec<Vec<Run>> = Vec::new();
    for run in spans.0 {
        match rows.last_mut() {
            Some(row) if row[0].top == run.top => row.push(run),
            _ => rows.push(vec![run]),
        }
    }
    rows
}
