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

const HALF_PERIOD: usize = 55;
const AMPLITUDE: i16 = 0x2000;

/// A square wave over `out`, one frame of two 16-bit little-endian samples
/// to each whole four bytes; a tail shorter than a frame is left alone.
pub fn fill(out: &mut [u8]) {
    for (i, frame) in out.chunks_exact_mut(4).enumerate() {
        let sample = if (i / HALF_PERIOD) & 1 == 0 { AMPLITUDE } else { -AMPLITUDE };
        let [lo, hi] = sample.to_le_bytes();
        frame.copy_from_slice(&[lo, hi, lo, hi]);
    }
}
