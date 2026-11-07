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

//! How a read of a guest's rows at a stride is cut into kernel calls
//! (MkPeerReadRows). Pure, so the host proofs hold it.

/// The most rows one call moves, the kernel's MAX_ROWS
/// (src/process/foreign/peer_rows_shape.rs); kernel_proofs holds the two
/// equal.
pub const ROWS_MAX: usize = 4096;

/// The most bytes one call moves, the kernel's MAX_SPAN; kernel_proofs holds
/// the two equal.
pub const ROWS_MAX_BYTES: usize = 1 << 20;

/// How many rows of `row` bytes one call moves; 0 when a row is empty or
/// alone more than a call moves, and is then read as its own span.
pub fn rows_a_call(row: usize) -> usize {
    match row {
        0 => 0,
        _ => (ROWS_MAX_BYTES / row).min(ROWS_MAX),
    }
}

/// The calls that read `count` rows of `row` bytes: each its first row and
/// how many rows it moves, in order, covering every row once. None when one
/// row is more than a call moves.
pub fn calls(row: usize, count: usize) -> Option<impl Iterator<Item = (usize, usize)>> {
    let per = rows_a_call(row);
    (per != 0).then(|| (0..count).step_by(per).map(move |first| (first, per.min(count - first))))
}
