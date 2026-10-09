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

use alloc::vec::Vec;

/* A partial repaint drawn off the surface. The compositor reads the surface
 * on another CPU whenever it composites, and a partial repaint draws the
 * app's whole content area: drawn in place, a composite in between (the
 * cursor moving over the window is one) showed the content half cleared,
 * a flicker under the pointer while a terminal printed or a list scrolled.
 * The paint now goes to a copy of the surface, and only the rows in `rows`
 * that changed are written back. Without the memory for the copy it draws
 * in place, as before. */
pub(super) fn draw_off_screen(
    surface: &mut [u32],
    row_words: usize,
    rows: core::ops::Range<usize>,
    draw: impl FnOnce(&mut [u32]),
) {
    let mut back: Vec<u32> = Vec::new();
    if row_words == 0 || back.try_reserve_exact(surface.len()).is_err() {
        draw(surface);
        return;
    }
    back.extend_from_slice(surface);
    draw(&mut back);
    let (start, end) = (rows.start * row_words, (rows.end * row_words).min(surface.len()));
    if start >= end {
        return;
    }
    for (dst, src) in
        surface[start..end].chunks_mut(row_words).zip(back[start..end].chunks(row_words))
    {
        if dst != src {
            dst.copy_from_slice(src);
        }
    }
}
