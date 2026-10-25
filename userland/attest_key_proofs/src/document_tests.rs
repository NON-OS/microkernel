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

//! The document carries the DMA posture after the completeness byte, then
//! the PCRs and log, and the key last, under version five.

use crate::security::attest_doc::document::{AttestationDoc, DOC_VERSION, IOMMU_INTEL_VTD};

pub(crate) fn doc() -> AttestationDoc {
    AttestationDoc {
        challenge: [7; 32],
        registry_root: [0xAB; 32],
        capsule_count: 3,
        registry_complete: true,
        iommu_vendor: IOMMU_INTEL_VTD,
        iommu_enforcing: true,
        unconfined_grants: 13,
        identity_devices: 5,
        pcrs: core::array::from_fn(|k| [0x30 + k as u8; 32]),
        log: b"log".to_vec(),
        attest: vec![1, 2, 3, 4],
        signature: vec![9; 72],
        ak_public: [0xCD; 64],
    }
}

/// Where the DMA posture ends, then the PCRs, the log and the blobs.
pub(crate) const PCRS_AT: usize = 8 + 4 + 32 + 32 + 4 + 1 + 1 + 1 + 4 + 4;
const ATTEST_LEN_AT: usize = PCRS_AT + 1 + 33 * 8 + 4 + 3;

#[test]
fn the_version_is_five_because_the_layout_grew() {
    assert_eq!(DOC_VERSION, 5);
    let b = doc().encode();
    assert_eq!(&b[8..12], &5u32.to_be_bytes());
}

#[test]
fn the_dma_posture_follows_the_completeness_byte() {
    let b = doc().encode();
    assert_eq!(b[81], IOMMU_INTEL_VTD);
    assert_eq!(b[82], 1);
    assert_eq!(&b[83..87], &13u32.to_be_bytes());
    assert_eq!(&b[87..91], &5u32.to_be_bytes(), "identity devices after the grants");
    assert_eq!(&b[ATTEST_LEN_AT..ATTEST_LEN_AT + 4], &4u32.to_be_bytes());
}

#[test]
fn the_key_is_the_last_blob_and_the_document_ends_with_it() {
    let b = doc().encode();
    let sig_len_at = ATTEST_LEN_AT + 4 + 4;
    let key_len_at = sig_len_at + 4 + 72;
    assert_eq!(&b[key_len_at..key_len_at + 4], &64u32.to_be_bytes());
    assert_eq!(&b[key_len_at + 4..], &[0xCD; 64]);
    assert_eq!(b.len(), key_len_at + 4 + 64);
}
