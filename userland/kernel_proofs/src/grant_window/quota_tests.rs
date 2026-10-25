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

//! A holder's quota is a bound, on count and on bytes, with no wrap.

use crate::proofs_1::quota::{spent, Spent, MAX_DMA_BYTES, MAX_DMA_GRANTS, MAX_MMIO_GRANTS};

const P: u64 = 4096;

#[test]
fn a_quota_is_a_bound() {
    let (n, b) = (MAX_DMA_GRANTS, MAX_DMA_BYTES);
    assert_eq!(spent(n - 1, 0, P, n, b), None);
    assert_eq!(spent(n, 0, P, n, b), Some(Spent::Grants), "one too many");
    assert_eq!(spent(0, b - P, P, n, b), None, "exactly full");
    assert_eq!(spent(0, b - P, 2 * P, n, b), Some(Spent::Bytes));
    assert_eq!(spent(0, u64::MAX, 1, usize::MAX, u64::MAX), Some(Spent::Bytes), "no wrap");
    assert_eq!(spent(n, b, P, n, b), Some(Spent::Grants), "the count is named first");
    let mmio = spent(MAX_MMIO_GRANTS, 0, 0, MAX_MMIO_GRANTS, u64::MAX);
    assert_eq!(mmio, Some(Spent::Grants), "MMIO counts too");
}
