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

use crate::browser::css::calc::{eval_value, V};

/* The largest font size kept. The glyph rasterizer allocates a coverage
 * buffer the size of one glyph's pixel box, about 5 MB at 1000px; a page
 * asking for more would exhaust the capsule heap on its first glyph. The
 * largest real display type (a 262px footer word mark) is far below. */
pub(super) const MAX_FONT_PX: f32 = 1000.0;

/* The absolute-size keywords, as the CSS Fonts 4 table sets them against a
 * 16px medium. */
const KEYWORDS: [(&str, f32); 8] = [
    ("xx-small", 9.0),
    ("x-small", 10.0),
    ("small", 13.0),
    ("medium", 16.0),
    ("large", 18.0),
    ("x-large", 24.0),
    ("xx-large", 32.0),
    ("xxx-large", 48.0),
];

/// A font-size in px against the parent's size `parent`: an absolute-size
/// keyword, smaller or larger (a 1.2 step), a length, a percentage of the
/// parent, or a math function mixing them. em and % both mean the parent's
/// size here. A negative size is invalid and drops the declaration; the
/// result stays within 1..=MAX_FONT_PX.
pub(super) fn font_size(value: &str, parent: f32) -> Option<f32> {
    let v = value.trim();
    let px = if let Some((_, px)) = KEYWORDS.iter().find(|(k, _)| v.eq_ignore_ascii_case(k)) {
        *px
    } else if v.eq_ignore_ascii_case("smaller") {
        parent / 1.2
    } else if v.eq_ignore_ascii_case("larger") {
        parent * 1.2
    } else {
        match eval_value(v, parent)? {
            V::Num(n) if n == 0.0 => 0.0,
            V::Num(_) => return None,
            V::Len { px, pml } => px + parent * pml / 1000.0,
            V::Math(m) => m.resolve(parent as i32) as f32,
        }
    };
    (px.is_finite() && px >= 0.0).then(|| px.clamp(1.0, MAX_FONT_PX))
}
