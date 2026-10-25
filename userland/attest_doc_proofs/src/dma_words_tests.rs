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

//! About's DMA row says "device DMA not confined by hardware" on exactly the
//! documents where device DMA is not held, and never on one where it is.

use crate::dma_words::{confined, words, CONFINED, UNCONFINED};

#[test]
fn the_owners_words_appear_on_exactly_the_unconfined_documents() {
    for enforcing in [false, true] {
        for grants in [0u32, 1, 2, 7, 8192, u32::MAX] {
            let held = enforcing && grants == 0;
            assert_eq!(confined(enforcing, grants), held, "{enforcing} {grants}");
            let want = if held { CONFINED } else { UNCONFINED };
            assert_eq!(words(enforcing, grants), want, "{enforcing} {grants}");
        }
    }
}

#[test]
fn the_words_are_the_owners_exactly() {
    assert_eq!(UNCONFINED, b"device DMA not confined by hardware");
    assert_ne!(CONFINED, UNCONFINED);
}

#[test]
fn the_no_iommu_posture_reads_unconfined_with_or_without_a_grant() {
    assert_eq!(words(false, 0), UNCONFINED, "posture taken, nothing claimed yet");
    assert_eq!(words(false, 3), UNCONFINED, "posture, devices claimed");
    assert_eq!(words(true, 1), UNCONFINED, "a unit in service and a grant beside it");
    assert_eq!(words(true, 0), CONFINED);
}

/// The vendor values the document names, each distinct, as the row's
/// evidence reads them.
#[test]
fn the_documents_vendor_values_are_distinct() {
    use crate::kernel_document::{IOMMU_AMD_VI, IOMMU_ARM_SMMU_V3, IOMMU_INTEL_VTD, IOMMU_NONE};
    let all = [IOMMU_NONE, IOMMU_INTEL_VTD, IOMMU_AMD_VI, IOMMU_ARM_SMMU_V3];
    assert_eq!(all, [0, 1, 2, 3]);
}
