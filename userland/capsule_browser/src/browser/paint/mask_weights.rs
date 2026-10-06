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

use super::grad::MaskLayer;

/// The mask coverage (0..=255) along screen row `y` from column `x0`, into
/// `m`, for a mask whose box sits at `o` ([x, y, w, h] on screen): 0 outside
/// the box, as mask-clip border-box has it. Layers intersect (multiply) or
/// add (a + b - ab), the two mask-composite modes pages use.
pub(super) fn weights(
    layers: &[MaskLayer],
    o: [i32; 4],
    isect: bool,
    (y, x0): (i32, i32),
    m: &mut [u32],
    tmp: &mut [u32],
) {
    let [ox, oy, w, h] = o;
    m.fill(0);
    let n = m.len() as i32;
    let (a, b) = ((ox - x0).clamp(0, n) as usize, (ox + w - x0).clamp(0, n) as usize);
    if y < oy || y >= oy + h || a >= b {
        return;
    }
    for (i, l) in layers.iter().enumerate() {
        l.row(y - oy, x0 + a as i32 - ox, &mut tmp[a..b]);
        for (mk, t) in m[a..b].iter_mut().zip(&tmp[a..b]) {
            let v = t >> 24;
            *mk = match (i, isect) {
                (0, _) => v,
                (_, true) => *mk * v / 255,
                (_, false) => *mk + v - *mk * v / 255,
            };
        }
    }
}

/// `old` and `new` mixed by coverage `m`: all of `new` at 255, `old` at 0.
pub(super) fn mix(old: u32, new: u32, m: u32) -> u32 {
    match m {
        0 => old,
        255.. => new,
        _ => {
            let ch = |s: u32| {
                let (o, n) = ((old >> s) & 0xff, (new >> s) & 0xff);
                ((o * (255 - m) + n * m + 127) / 255) << s
            };
            (new & 0xff00_0000) | ch(16) | ch(8) | ch(0)
        }
    }
}
