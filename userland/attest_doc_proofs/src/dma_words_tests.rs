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
//! documents whose DMA is not held, read through the real parser.

use crate::dma_words::{words, CONFINED, UNCONFINED};

#[test]
fn the_owners_words_appear_on_exactly_the_unconfined_documents() {
    for enforcing in [false, true] {
        for grants in [0u32, 1, 7, u32::MAX] {
            for identity in [0u32, 1, 9] {
                let held = held_doc(enforcing, grants, identity);
                assert_eq!(held, enforcing && grants == 0 && identity == 0);
                let want = if held { CONFINED } else { UNCONFINED };
                assert_eq!(words(held), want, "{enforcing} {grants} {identity}");
            }
        }
    }
}

#[test]
fn the_words_are_the_owners_exactly() {
    assert_eq!(UNCONFINED, b"device DMA not confined by hardware");
    assert_eq!(words(false), UNCONFINED);
    assert_eq!(words(true), CONFINED);
    assert_ne!(CONFINED, UNCONFINED);
}

/// Whether About holds DMA for a document the kernel encoded, read through
/// the real parser.
fn held_doc(enforcing: bool, grants: u32, identity: u32) -> bool {
    use crate::doc_build::{encoded, CHALLENGE, IDENTITY_AT};
    use crate::doc_parse::parse;
    use crate::kernel_document::{IOMMU_INTEL_VTD, IOMMU_NONE};
    let vendor = if enforcing { IOMMU_INTEL_VTD } else { IOMMU_NONE };
    let mut v = encoded(&CHALLENGE, &[1], &[9; 8], 3, vendor, enforcing, grants);
    v[IDENTITY_AT..IDENTITY_AT + 4].copy_from_slice(&identity.to_be_bytes());
    parse(&v, &CHALLENGE).unwrap().dma_held()
}
