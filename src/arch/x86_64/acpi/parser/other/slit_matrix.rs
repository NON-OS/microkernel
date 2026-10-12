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

//! SLIT's distance matrix from the table's bytes (ACPI 6.5, 5.2.17): the
//! locality count (u64 after the SDT header), then count × count bytes,
//! row by row, the distance from each locality to each other. A locality
//! is to itself 10; values below 10 are reserved and refuse the table, and
//! 255 is unreachable. The one SLIT reader: the aarch64 tables and
//! `kernel_proofs` mount this file by path.

use alloc::vec::Vec;

pub const COUNT_AT: usize = 36;
pub const MATRIX_AT: usize = 44;
/// Localities read; past this the table is refused rather than cut.
pub const MAX_LOCALITIES: usize = 256;
pub const LOCAL: u8 = 10;

/// The locality count and the matrix, or None for a table that does not
/// hold what it says or holds a reserved distance.
pub fn matrix(table: &[u8]) -> Option<(usize, Vec<u8>)> {
    let count = u64::from_le_bytes(table.get(COUNT_AT..MATRIX_AT)?.try_into().ok()?);
    let n = usize::try_from(count).ok().filter(|n| (1..=MAX_LOCALITIES).contains(n))?;
    let m = table.get(MATRIX_AT..MATRIX_AT + n * n)?;
    let sound = |(i, d): (usize, &u8)| if i / n == i % n { *d == LOCAL } else { *d > LOCAL };
    m.iter().enumerate().all(sound).then(|| (n, m.to_vec()))
}
