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

//! XRGB made ARGB: every alpha byte set, the colour left alone.

use crate::level::level;
#[cfg(target_arch = "x86_64")]
use crate::level::Level;

pub fn opaque_row(row: &mut [u8]) {
    let n = row.len() / 4 * 4;
    let row = &mut row[..n];
    // SAFETY: the level says the processor runs these instructions.
    let done = match level() {
        #[cfg(target_arch = "x86_64")]
        Level::Avx2 => unsafe { crate::avx2::opaque(row) },
        #[cfg(target_arch = "x86_64")]
        Level::Sse2 => unsafe { crate::sse2::opaque(row) },
        _ => 0,
    };
    scalar(&mut row[done..]);
}

pub fn scalar(row: &mut [u8]) {
    row.chunks_exact_mut(4).for_each(|px| px[3] = 0xFF);
}
