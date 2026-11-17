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
//! A surface that records where a row's text and icons go and paints
//! nothing.

use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{Rect, Row, Surface};

/// What a row draws: each run of text as its left edge and top, then each
/// icon's square.
#[derive(Default)]
pub(crate) struct Marks(pub(crate) Vec<(i32, i32)>, pub(crate) Vec<Rect>);

impl Surface for Marks {
    fn fill_rect(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn blend_rect(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn fill_round(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn stroke_round(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn circle(&mut self, _: u32, _: u32, _: u32, _: u32) {}
    fn text_ttf(&mut self, x: i32, top: i32, _: &str, _: u32, _: f32) -> i32 {
        self.0.push((x, top));
        x
    }
    fn icon(&mut self, _: IconId, x: u32, y: u32, size: u32, _: u32) {
        self.1.push(Rect { x, y, w: size, h: size });
    }
}

/// What `r` draws in `at`.
pub(crate) fn marks(r: Row<'static>, at: Rect) -> Marks {
    let mut m = Marks::default();
    r.draw(&mut m, at);
    m
}
