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

use alloc::vec;
use alloc::vec::Vec;

use super::axis::Axis;
use super::normalize::normalize;

const ONE: u64 = 1 << 16;

// Box filter with fractional coverage of the first and last cells.
pub fn area(src: u32, off: u64, span: u64, dst: u32, taps: usize) -> Axis {
    let mut start = Vec::with_capacity(dst as usize);
    let mut weight = vec![0u16; dst as usize * taps];
    let mut raw = vec![0u64; taps];
    let last_cell = src as u64 - 1;
    for i in 0..dst as u64 {
        let lo = off + (i * span) / dst as u64;
        let hi = (off + ((i + 1) * span) / dst as u64).max(lo + 1);
        let first = (lo / ONE).min(last_cell);
        for (k, r) in raw.iter_mut().enumerate() {
            let cell = first + k as u64;
            let c0 = cell * ONE;
            let a = lo.max(c0);
            let b = hi.min(c0 + ONE);
            *r = if cell <= last_cell && b > a { b - a } else { 0 };
        }
        let at = i as usize * taps;
        normalize(&raw, &mut weight[at..at + taps]);
        start.push(first as u32);
    }
    Axis { start, weight, taps }
}
