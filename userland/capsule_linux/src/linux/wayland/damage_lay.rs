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

//! The part of a client's pixels it said changed, laid into its window.
//! Pure, so the host proofs hold it.

/// The box `r` (x0, y0, x1, y1) of `client`, `stride` bytes a row, laid
/// into `out`, `row` bytes a row, at (`cx`, `cy`): copied, or flattened
/// onto `ground` when one is given. False when a row of it runs past
/// either, or its offset does not fit, the rows before it laid.
pub fn lay(
    out: &mut [u8],
    (row, cx, cy): (u32, u32, u32),
    client: &[u8],
    stride: u32,
    r: (u32, u32, u32, u32),
    ground: Option<u32>,
) -> bool {
    let (x0, y0, x1, y1) = r;
    let n = (x1.saturating_sub(x0) as usize).saturating_mul(4);
    let (cx, cy) = (cx as usize, cy as usize);
    for y in y0 as usize..y1 as usize {
        let from = span(y, stride, x0 as usize, n);
        let to = cy.checked_add(y).zip(cx.checked_add(x0 as usize));
        let to = to.and_then(|(y, x)| span(y, row, x, n));
        let src = from.and_then(|f| client.get(f));
        let (Some(src), Some(dst)) = (src, to.and_then(|t| out.get_mut(t))) else {
            return false;
        };
        match ground {
            Some(g) => nonos_pixel_rows::over_colour(dst, src, g),
            None => dst.copy_from_slice(src),
        }
    }
    true
}

/// The `n` bytes from pixel `x` of row `y`, rows `stride` bytes apart; None
/// when an offset does not fit.
fn span(y: usize, stride: u32, x: usize, n: usize) -> Option<core::ops::Range<usize>> {
    let start = y.checked_mul(stride as usize)?.checked_add(x.checked_mul(4)?)?;
    Some(start..start.checked_add(n)?)
}
