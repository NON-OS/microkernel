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

use nonos_toolkit::font::ttf::{ascent_with, builtin_face, line_height_with, measure_with};

use super::measure::{FixedMeasurer, Measurer};
use super::style::{Family, RunStyle};

/*
 * Without a built-in face the advances would all be zero, every line would
 * take the whole block and the caret could not move across it; the fixed
 * advances keep the page laid out until the face is there.
 */
pub struct TtfMeasurer;

impl Measurer for TtfMeasurer {
    fn advance(&self, text: &str, style: &RunStyle) -> f32 {
        let mono = style.family == Family::Mono;
        match builtin_face(mono, style.bold) {
            Some(f) => measure_with(f, text, style.size_px) as f32,
            None => FixedMeasurer.advance(text, style),
        }
    }

    fn line_height(&self, style: &RunStyle) -> f32 {
        let mono = style.family == Family::Mono;
        match builtin_face(mono, style.bold) {
            Some(f) => line_height_with(f, style.size_px) as f32,
            None => FixedMeasurer.line_height(style),
        }
    }

    fn ascent(&self, style: &RunStyle) -> f32 {
        let mono = style.family == Family::Mono;
        match builtin_face(mono, style.bold) {
            Some(f) => ascent_with(f, style.size_px) as f32,
            None => FixedMeasurer.ascent(style),
        }
    }
}
