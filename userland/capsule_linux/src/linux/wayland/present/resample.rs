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

//! A buffer's pixels brought to the size they are shown at, nearest
//! neighbour: each shown pixel takes the buffer's pixel under its centre.
//! Pure, so the host proofs hold it.

/// `src`, `sw` by `sh` pixels at `stride` bytes a row, into `dst` at `dw`
/// by `dh` pixels packed four bytes each. False, with nothing written, when
/// either slice is short of its shape or either shape is empty.
pub fn resample(
    src: &[u8],
    (sw, sh, stride): (u32, u32, u32),
    dst: &mut [u8],
    (dw, dh): (u32, u32),
) -> bool {
    let (sw, sh, stride, dw, dh) =
        (sw as usize, sh as usize, stride as usize, dw as usize, dh as usize);
    if sw == 0 || sh == 0 || dw == 0 || dh == 0 || stride < sw * 4 {
        return false;
    }
    if src.len() < (sh - 1) * stride + sw * 4 || dst.len() < dw * dh * 4 {
        return false;
    }
    for (y, out) in dst.chunks_exact_mut(dw * 4).take(dh).enumerate() {
        let row = ((2 * y + 1) * sh / (2 * dh)).min(sh - 1) * stride;
        for (x, px) in out.chunks_exact_mut(4).enumerate() {
            let at = row + ((2 * x + 1) * sw / (2 * dw)).min(sw - 1) * 4;
            px.copy_from_slice(&src[at..at + 4]);
        }
    }
    true
}
