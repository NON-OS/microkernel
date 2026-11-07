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

//! The rows one strided peer read moves, checked before any guest page is
//! touched. Pure, so the host proofs hold it (kernel_proofs peer_rows).

/// At most this many rows a call, so the pages one call walks stay bounded
/// however narrow its rows are: a row touches at most two pages more than
/// its bytes fill.
pub(crate) const MAX_ROWS: u64 = 4096;

/// `count` rows of `row` bytes each, row `i` at `i * stride` bytes from the
/// first, at the same offsets on the guest's side and the caller's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Rows {
    pub(crate) row: u64,
    pub(crate) count: u64,
    pub(crate) stride: u64,
}

impl Rows {
    /// The layout, or None when a row is empty or wider than its stride,
    /// there are no rows or more than `MAX_ROWS`, the bytes the rows hold
    /// pass `max_bytes`, or the span from the first byte to the last does
    /// not fit in a u64.
    pub(crate) fn new(row: u64, count: u64, stride: u64, max_bytes: u64) -> Option<Rows> {
        if row == 0 || row > stride || count == 0 || count > MAX_ROWS {
            return None;
        }
        if row.checked_mul(count)? > max_bytes {
            return None;
        }
        let r = Rows { row, count, stride };
        r.span()?;
        Some(r)
    }

    /// The bytes from the first row's start to the last row's end.
    pub(crate) fn span(&self) -> Option<u64> {
        self.stride.checked_mul(self.count.checked_sub(1)?)?.checked_add(self.row)
    }

    /// Where row `i` starts, from the first; None past the last.
    pub(crate) fn at(&self, i: u64) -> Option<u64> {
        if i >= self.count {
            return None;
        }
        self.stride.checked_mul(i)
    }

    /// The bytes all the rows hold.
    pub(crate) fn bytes(&self) -> u64 {
        self.row.saturating_mul(self.count)
    }
}
