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

//! Dialog: the runs of text a dialog draws, with where each line sits, so
//! a proof can read back the title, the body's lines, the note and the
//! labels and see that no two of them share a row.

use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::paint::measure_ttf;
use nonos_toolkit::widgets::{Dialog, Rect, Surface};

/// One run of text: what it says, and its line's top and bottom.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Line {
    pub(crate) text: String,
    pub(crate) top: i32,
    pub(crate) bottom: i32,
}

/// A surface that keeps the lines of text drawn on it and paints nothing.
struct Lines(Vec<Line>);

impl Surface for Lines {
    fn fill_rect(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn blend_rect(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn fill_round(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn stroke_round(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn circle(&mut self, _: u32, _: u32, _: u32, _: u32) {}
    fn text_ttf(&mut self, x: i32, top: i32, text: &str, _: u32, px: f32) -> i32 {
        let bottom = top + line_height(px);
        self.0.push(Line { text: text.to_string(), top, bottom });
        x + measure_ttf(text, px)
    }
    fn icon(&mut self, _: IconId, _: u32, _: u32, _: u32, _: u32) {}
}

/// The lines `d` draws in `at`, in order: title, body, note, labels.
pub(crate) fn spans(d: Dialog, at: Rect) -> Vec<Line> {
    let mut lines = Lines(Vec::new());
    d.draw(&mut lines, at);
    lines.0
}
