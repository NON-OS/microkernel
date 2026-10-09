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
use super::residuals::Nz;

/// The four U then four V blocks of a macroblock, dequantized with the
/// chroma steps `q`; true when any carries a coefficient.
pub(super) fn chroma(
    br: &mut Bools,
    probs: &[u8],
    q: [i32; 2],
    (top, left): (&mut Nz, &mut Nz),
    c: &mut [i16; 384],
) -> bool {
    let mut any = false;
    for (plane, base) in [(0, 256), (1, 320)] {
        let (t, l) = if plane == 0 { (&mut top.u, &mut left.u) } else { (&mut top.v, &mut left.v) };
        for (y, ly) in l.iter_mut().enumerate() {
            for (x, tx) in t.iter_mut().enumerate() {
                let at = base + (y * 2 + x) * 16;
                let nz = block_tokens(
                    br,
                    probs,
                    (2, *tx as usize + *ly as usize),
                    q,
                    0,
                    &mut c[at..at + 16],
                );
                (*tx, *ly) = (nz > 0, nz > 0);
                any |= nz > 1 || c[at] != 0;
            }
        }
    }
    any
}
