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

use crate::browser::css::calc::split_top::words;
use crate::browser::css::computed::GridTrack;
use crate::browser::css::one_track::one_track;

/* A grid template value split at its slash (outside quotes and brackets):
 * the rows half, and the columns half when there is one. */
pub(super) fn halves(v: &str) -> (&str, Option<&str>) {
    let (mut depth, mut quote) = (0i32, false);
    for (i, b) in v.bytes().enumerate() {
        match b {
            b'"' | b'\'' => quote = !quote,
            b'(' | b'[' if !quote => depth += 1,
            b')' | b']' if !quote => depth -= 1,
            b'/' if !quote && depth == 0 => return (&v[..i], Some(&v[i + 1..])),
            _ => {}
        }
    }
    (v, None)
}

/* In grid's auto-flow form, the implicit track size after the keyword. */
pub(super) fn auto_flow(half: &str, fs: u32) -> Option<GridTrack> {
    let size = || words(half).filter(|w| !matches!(*w, "auto-flow" | "dense")).last();
    half.contains("auto-flow")
        .then(|| size().and_then(|t| one_track(t, fs)).unwrap_or(GridTrack::Auto))
}
