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

//! The kernel's admission code, the files the spawn gate runs.

#[path = "../../../src/security/admission_cache/mod.rs"]
pub mod admission_cache;

// The kernel files re-export more than the spawn gate reads.
#[allow(unused_imports, dead_code)]
#[path = "../../../src/security/nonos_trust_anchor"]
pub mod nonos_trust_anchor {
    mod cursor;
    mod decode;
    mod error;
    mod schema;

    pub use decode::decode;
    pub use error::TrustAnchorDecodeError;
    pub use schema::NonosTrustAnchorPolicy;
}

#[allow(unused_imports, dead_code)]
#[path = "../../../src/security/nonos_id_cert/mod.rs"]
pub mod nonos_id_cert;

#[allow(unused_imports, dead_code)]
#[path = "../../../src/security/capsule_manifest/mod.rs"]
pub mod capsule_manifest;

#[allow(dead_code)]
#[path = "../../../src/security/capsule_attest"]
pub mod capsule_attest {
    pub mod error;
    mod layout;
    pub(crate) mod measure;
    mod path;

    /// The spawn gate's check against one root, as `against_root` calls it.
    pub fn verify_against(
        trailer: &[u8],
        digest: &[u8; 32],
        granted_caps: u64,
        root: &[u8; 32],
    ) -> Result<[u8; 32], error::AttestError> {
        path::verify_against(trailer, digest, granted_caps, root)
    }
}
