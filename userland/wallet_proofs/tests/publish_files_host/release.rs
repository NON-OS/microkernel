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

//! A schema 3 release as large as its capabilities let it be (64 of 128
//! bytes), the same length whatever its number below 1000.

use nonos_marketplace_abi::{CapsuleRelease, ReleaseExt, ValidationReport, ValidationStatus};

pub fn release(i: usize) -> CapsuleRelease {
    let ext = ReleaseExt {
        commerce_hash: [0xc0; 32],
        guest_policy: vec![],
        chunk_root: [0xcc; 32],
        content_len: 4096,
        proof_anchors: vec![],
        release_attest_sig: [0; 65],
    };
    let status = ValidationStatus::from_u8(0);
    let validation = ValidationReport {
        status,
        note: String::new(),
        validator_id: String::new(),
        validated_at_ms: 0,
    };
    CapsuleRelease {
        release_id: format!("r@{i:03}"),
        manifest_hash: [1; 32],
        package_hash: [0x9a; 32],
        package_url: "https://h/p".into(),
        publisher_signature: vec![],
        supported_arches: vec![],
        kernel_abi_min: 1,
        required_capabilities: vec!["c".repeat(128); 64],
        zk_trailer_hash: [0; 32],
        ext: Some(ext),
        validation,
    }
}
