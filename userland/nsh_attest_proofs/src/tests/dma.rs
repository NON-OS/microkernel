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

//! Step 6: an IOMMU is present and enforcing, nothing goes around it, and
//! no device is left reaching all of memory.

use nsh_attest::Refusal;

use super::{passes, refused};
use crate::build::Spec;

#[test]
fn dma_that_is_not_confined_is_refused() {
    let bad = [(0, true, 0, 0), (0, false, 0, 0), (1, false, 0, 0), (1, true, 1, 0)];
    let more = [(2, true, u32::MAX, 0), (1, true, 0, 1), (3, true, 0, u32::MAX)];
    for dma in bad.into_iter().chain(more) {
        assert_eq!(refused(Spec { dma, ..Spec::default() }), Refusal::Dma, "{dma:?}");
    }
}

#[test]
fn every_enforcing_iommu_is_accepted() {
    for vendor in 1..=3 {
        passes(Spec { dma: (vendor, true, 0, 0), ..Spec::default() });
    }
}
