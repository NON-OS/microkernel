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
/// with `blend` into `out`, `row` bytes a row, at (`cx`, `cy`).
pub fn lay(
    out: &mut [u8],
    (row, cx, cy): (u32, u32, u32),
    client: &[u8],
    stride: u32,
    r: (u32, u32, u32, u32),
    blend: impl Fn(u32) -> u32,
) {
    let (x0, y0, x1, y1) = r;
    let n = (x1.saturating_sub(x0) * 4) as usize;
    for y in y0..y1 {
        let from = (y * stride + x0 * 4) as usize;
        let to = ((cy + y) * row + (cx + x0) * 4) as usize;
        let (Some(src), Some(dst)) = (client.get(from..from + n), out.get_mut(to..to + n)) else {
            return;
        };
        for (d, s) in dst.chunks_exact_mut(4).zip(src.chunks_exact(4)) {
            d.copy_from_slice(&blend(u32::from_le_bytes([s[0], s[1], s[2], s[3]])).to_le_bytes());
        }
    }
}
