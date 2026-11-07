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

//! The rows a new frame changed from the one on screen.
//!
//! A whole repaint draws every pixel of the desk and the chrome again, and
//! each is the size of the screen. Shown whole, each one has the compositor
//! compose the whole screen, once for the desk and once for the chrome, even
//! when only the menu bar or the dock changed. Held to the frame on screen,
//! it is shown as the band of rows that differ, and not at all when none do.

/// The first row where `next` and `shown` differ and the row past the last,
/// for frames of `row_words` pixels a row; None when no row differs. Rows
/// past the end of the shorter frame are not compared.
pub fn changed_rows(next: &[u32], shown: &[u32], row_words: usize) -> Option<(usize, usize)> {
    if row_words == 0 {
        return None;
    }
    let pairs = || next.chunks_exact(row_words).zip(shown.chunks_exact(row_words));
    let first = pairs().position(|(a, b)| a != b)?;
    let last = pairs().rposition(|(a, b)| a != b)?;
    Some((first, last + 1))
}

/// The rows the frame at `next` changed from the one at `shown`, both mapped
/// surfaces of `words` pixels, `row_words` a row; None when the two are one
/// backing or no row differs.
pub fn rows_apart(next: u64, shown: u64, words: usize, row_words: usize) -> Option<(u32, u32)> {
    if next == 0 || shown == 0 || next == shown {
        return None;
    }
    // SAFETY: `next` and `shown` are the shell's two mapped backings of one
    // surface, `words` pixels each, the span every painter writes, and they
    // are not the same backing. Nothing writes either while this reads them:
    // the shell paints on one thread and the compositor maps both read only.
    let (a, b) = unsafe {
        (
            core::slice::from_raw_parts(next as *const u32, words),
            core::slice::from_raw_parts(shown as *const u32, words),
        )
    };
    let (first, end) = changed_rows(a, b, row_words)?;
    Some((u32::try_from(first).ok()?, u32::try_from(end).ok()?))
}

/// The part of a surface `width` wide that rows `rows` cover: the screen's
/// width, from the first changed row to the row past the last. None when no
/// row changed.
pub fn band(rows: Option<(u32, u32)>, width: u32) -> Option<(u32, u32, u32, u32)> {
    let (first, end) = rows?;
    (end > first && width > 0).then(|| (0, first, width, end - first))
}
