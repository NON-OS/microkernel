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

use super::axis::UNIT;

// Scale raw weights so they sum to exactly UNIT: a flat colour stays flat.
pub fn normalize(raw: &[u64], out: &mut [u16]) {
    let total: u64 = raw.iter().sum();
    if total == 0 {
        if let Some(first) = out.first_mut() {
            *first = UNIT as u16;
        }
        return;
    }
    let mut given = 0u32;
    let mut largest = 0usize;
    for (k, (&r, w)) in raw.iter().zip(out.iter_mut()).enumerate() {
        *w = ((r * UNIT as u64) / total) as u16;
        given += *w as u32;
        if r > raw[largest] {
            largest = k;
        }
    }
    // Rounding down leaves a remainder; the heaviest tap absorbs it.
    out[largest] += (UNIT - given) as u16;
}
