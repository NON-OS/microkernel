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

use crate::browser::css::Computed;

/* What sits before an inline item: the width of the collapsed space that
 * precedes it (0 when the source had none there), whether a line may
 * break before it, whether that space lies inside underlined text, and
 * whether the item continues the word before it (a piece cut only to
 * allow a break, painted as one word with it when the line holds both). */
#[derive(Clone, Copy, Default)]
pub(in super::super) struct Lead {
    pub space: i32,
    pub brk: bool,
    pub ul: bool,
    pub join: bool,
}

/* The style a word paints with, as its fragment carries it. */
#[derive(Clone, Copy)]
pub(in super::super) struct Ink {
    pub px: f32,
    pub color: u32,
    pub bg: u32,
    pub bold: bool,
    pub mono: bool,
    pub underline: bool,
    pub font: u32,
    pub spacing: f32,
    pub italic: bool,
}

impl Ink {
    /* The paint style of text in `s`, over background `bg`. */
    pub(in super::super) fn of(s: &Computed, bg: u32, underline: bool) -> Ink {
        let (px, color, bold, mono, font) = (s.font_px, s.color, s.bold, s.mono, s.font_key);
        let (spacing, italic) = (s.letter_spacing, s.italic);
        Ink { px, color, bg, bold, mono, underline, font, spacing, italic }
    }
}
