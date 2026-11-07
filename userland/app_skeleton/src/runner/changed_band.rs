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

//! The rows a frame drawn whole changed from the frame on screen. The
//! buffer a window's layer shows holds the whole picture the compositor
//! composes from (swap_paint.rs), so a frame drawn whole into the other
//! buffer that differs from it only in some rows needs only those rows
//! composed: a command's output in the Terminal, not its whole window.

use super::changed_runs::runs_in;
use crate::swap::{Damage, Part, MAX_PARTS};

/// The first row where `next` and `shown` differ and the row past the
/// last, in frames of `rows` rows `row_words` pixels apart; None when no
/// row differs, or when either is too short to hold `rows` rows.
pub fn changed_rows(
    next: &[u32],
    shown: &[u32],
    row_words: usize,
    rows: usize,
) -> Option<(usize, usize)> {
    let words = row_words.checked_mul(rows)?;
    let (a, b) = (next.get(..words)?, shown.get(..words)?);
    if row_words == 0 {
        return None;
    }
    let pairs = || a.chunks_exact(row_words).zip(b.chunks_exact(row_words));
    let first = pairs().position(|(x, y)| x != y)?;
    let last = pairs().rposition(|(x, y)| x != y)?;
    Some((first, last + 1))
}

/// What a frame `width` wide whose rows `rows` changed presents: those
/// rows, the window wide. None when no row changed.
pub fn band(rows: Option<(usize, usize)>, width: u32) -> Option<Damage> {
    let (first, end) = rows?;
    let y = u32::try_from(first).ok()?;
    let height = u32::try_from(end.checked_sub(first)?).ok()?;
    (height > 0 && width > 0).then_some(Damage { x: 0, y, width, height })
}

/// What a frame drawn whole presents.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shown {
    /// No row changed: nothing.
    Nothing,
    /// One box.
    Band(Damage),
    /// The band round every changed row, and the first `usize` of the boxes
    /// inside it that changed, each repainted alone.
    Parts(Damage, [Part; MAX_PARTS], usize),
}

/// What a frame drawn whole into `next` presents against the `shown` one,
/// both of `rows` rows `row_words` pixels apart, `width` by `rows` on
/// screen: nothing when it changed no row; the band of the rows it changed
/// when they are one run (changed_runs.rs), and the band with each run
/// inside it when they are more; the whole frame when either buffer cannot
/// be read as that many rows, so a frame is never held back by a buffer it
/// could not compare.
pub fn present_rows(
    next: &[u32],
    shown: &[u32],
    (row_words, rows): (usize, usize),
    (width, height): (u32, u32),
) -> Shown {
    let whole = Shown::Band(Damage { x: 0, y: 0, width, height });
    let words = row_words.checked_mul(rows);
    let read = |b: &[u32]| words.is_some_and(|n| b.len() >= n);
    if row_words == 0 || !read(next) || !read(shown) {
        return whole;
    }
    let span = changed_rows(next, shown, row_words, rows);
    let (Some(span), Some(d)) = (span, band(span, width)) else { return Shown::Nothing };
    let runs = runs_in(next, shown, row_words, span);
    let mut parts = [(0, 0, 0, 0); MAX_PARTS];
    for (p, r) in parts.iter_mut().zip(runs.iter()) {
        match band(Some(r), width) {
            Some(b) => *p = (b.x, b.y, b.width, b.height),
            None => return Shown::Band(d),
        }
    }
    match runs.count() {
        0 | 1 => Shown::Band(d),
        n => Shown::Parts(d, parts, n),
    }
}
