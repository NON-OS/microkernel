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

use super::bits::Bools;
use super::coeffs::block_tokens;
use super::idct::wht;
use super::residuals_uv::chroma;

/// Which 4x4 blocks along one macroblock edge had nonzero coefficients:
/// the token context its neighbour reads.
#[derive(Clone, Copy, Default)]
pub(super) struct Nz {
    pub y: [bool; 4],
    pub u: [bool; 2],
    pub v: [bool; 2],
    pub dc: bool,
}

/// A macroblock's residual coefficients (RFC 6386 13), dequantized with
/// `q` into `c`: 16 luma blocks then 4 U and 4 V, 16 each. A 16x16
/// predicted block first reads the second-order DC block and spreads its
/// inverse WHT into the luma DCs. Returns whether any block carries a
/// coefficient past its DC or a nonzero DC, which is what turns on the
/// inner-edge loop filter.
pub(super) fn residuals(
    br: &mut Bools,
    probs: &[u8],
    q: &[i32; 6],
    i4x4: bool,
    (top, left): (&mut Nz, &mut Nz),
    c: &mut [i16; 384],
) -> bool {
    c.fill(0);
    let (first, ytype) = if i4x4 { (0, 3) } else { (1, 0) };
    let mut any = false;
    if !i4x4 {
        let mut dc = [0i16; 16];
        let ctx = top.dc as usize + left.dc as usize;
        let nz = block_tokens(br, probs, (1, ctx), [q[2], q[3]], 0, &mut dc);
        (top.dc, left.dc) = (nz > 0, nz > 0);
        wht(&dc, c);
    }
    for y in 0..4 {
        for x in 0..4 {
            let ctx = top.y[x] as usize + left.y[y] as usize;
            let blk = &mut c[(y * 4 + x) * 16..(y * 4 + x + 1) * 16];
            let nz = block_tokens(br, probs, (ytype, ctx), [q[0], q[1]], first, blk);
            (top.y[x], left.y[y]) = (nz > first, nz > first);
            any |= nz > 1 || blk[0] != 0;
        }
    }
    chroma(br, probs, [q[4], q[5]], (top, left), c) | any
}
