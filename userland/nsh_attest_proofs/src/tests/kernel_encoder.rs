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

//! The kernel's own encoder (`src/security/attest_doc/document_encode.rs`)
//! writes the document this client reads: a valid host's document, taken
//! apart and written again by the kernel, is the same bytes and passes the
//! whole check. A layout the kernel writes and the client does not read
//! fails here, not on a machine.

use nsh_attest::consts::QUOTED_PCRS_V4;
use nsh_attest::Doc;

use super::passes;

mod large;
use crate::build::Spec;
use crate::kernel_document::{AttestationDoc, DOC_VERSION, MAX_LOG, QUOTED_PCRS};

/// The document as the kernel's encoder writes these values.
fn by_the_kernel(bytes: &[u8]) -> Vec<u8> {
    let d = Doc::parse(bytes).expect("a valid document");
    AttestationDoc {
        challenge: *d.challenge,
        registry_root: *d.registry_root,
        capsule_count: d.capsule_count,
        registry_complete: d.registry_complete,
        iommu_vendor: d.iommu_vendor,
        iommu_enforcing: d.iommu_enforcing,
        unconfined_grants: d.unconfined_grants,
        identity_devices: d.identity_devices,
        pcrs: d.pcrs.map(|p| *p),
        log: d.log.to_vec(),
        attest: d.attest.to_vec(),
        signature: d.signature.to_vec(),
        ak_public: *d.ak_public,
    }
    .encode()
}

#[test]
fn the_kernel_and_the_client_agree_on_every_number() {
    assert_eq!(DOC_VERSION, nsh_attest::consts::DOC_VERSION);
    assert_eq!(QUOTED_PCRS, QUOTED_PCRS_V4);
    assert_eq!(MAX_LOG, nsh_attest::consts::MAX_LOG);
}

#[test]
fn a_document_the_kernel_encodes_is_byte_for_byte_the_one_the_client_reads() {
    for s in [Spec::default(), Spec { dma: (3, true, 0, 0), ..Spec::default() }] {
        let b = s.build();
        assert_eq!(by_the_kernel(&b.doc), b.doc);
    }
}

#[test]
fn a_document_the_kernel_encodes_passes_the_whole_check() {
    passes(Spec { bytes: |d| *d = by_the_kernel(d), ..Spec::default() });
}
