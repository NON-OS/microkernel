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
//! Placing a field's caret from a click.

use core::iter::once;

use super::geometry::reach;
use super::Field;
use crate::decorations::Rect;

impl Field<'_> {
    /// The byte index into the text, always a char boundary, where a click at
    /// `x` puts the caret: the boundary of the shown text nearest to `x`,
    /// the earlier on a tie, measured and scrolled as `draw` lays the field
    /// out in `at`, so the caret is drawn where the click fell. A click on
    /// the ellipsis before a kept tail takes the tail's start, one on the
    /// ellipsis after a truncated head takes the end, and an empty field
    /// takes 0. `x` uses `draw`'s coordinates, as `hit` does.
    pub fn caret_at(&self, at: Rect, x: i32) -> usize {
        let p = self.parts(at);
        let shown = p.view.shown.as_str();
        let want = i64::from(x) - i64::from(p.text.x);
        let ends = shown.char_indices().map(|(i, _)| i).chain(once(shown.len()));
        let off = |k: usize| (i64::from(reach(shown, k, p.text.w, p.px)) - want).abs();
        let k = ends.min_by_key(|&k| off(k)).unwrap_or(0);
        p.view.text_at(k, self.text.len())
    }
}
