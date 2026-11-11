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

//! The attestation document a TPM signed, as About's Verify screen reads it.

/// The parsed document's shape, as the parser hands it over.
#[path = "."]
pub mod doc_parse {
    #[path = "../../capsule_about/src/about/data/doc_parse/types.rs"]
    mod types;
    pub use types::Doc;
}

/// The one document this window asks for.
pub mod attest_doc {
    use super::doc_parse::Doc;

    pub enum Attestation {
        NotAsked,
        Refused,
        Malformed,
        Produced(Doc),
    }

    pub fn request() -> Attestation {
        Attestation::Produced(Doc {
            registry_root: [0xab; 32],
            capsule_count: 44,
            registry_complete: true,
            iommu_vendor: 1,
            iommu_enforcing: true,
            unconfined_grants: 0,
            challenge_echoed: true,
            attest_len: 143,
            signature_len: 72,
        })
    }
}
