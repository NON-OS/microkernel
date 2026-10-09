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

use super::axis::Axis;

// Add one source line, filtered horizontally and weighted by `wy`, into
// the destination row's per-channel accumulators.
pub fn accumulate(line: &[u32], ax: &Axis, wy: u64, acc: &mut [u64]) {
    for (dx, out) in acc.chunks_mut(3).enumerate() {
        let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
        let first = ax.start[dx] as usize;
        let weights = &ax.weight[dx * ax.taps..(dx + 1) * ax.taps];
        for (k, &w) in weights.iter().enumerate() {
            let Some(&p) = line.get(first + k) else {
                break;
            };
            let w = w as u32;
            r += w * ((p >> 16) & 0xFF);
            g += w * ((p >> 8) & 0xFF);
            b += w * (p & 0xFF);
        }
        out[0] += r as u64 * wy;
        out[1] += g as u64 * wy;
        out[2] += b as u64 * wy;
    }
}
