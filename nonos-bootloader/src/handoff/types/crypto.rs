// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::security::AttestPolicy;

/// Cryptographic verification results from boot process.
#[derive(Clone, Copy)]
pub struct CryptoHandoff {
    pub signature_valid: bool,
    pub secure_boot: bool,
    pub kernel_hash: [u8; 32],
    pub zk_attested: bool,
    pub zk_program_hash: [u8; 32],
    pub zk_capsule_commitment: [u8; 32],
    pub policy: AttestPolicy,
}

impl Default for CryptoHandoff {
    fn default() -> Self {
        Self {
            signature_valid: false,
            secure_boot: false,
            kernel_hash: [0u8; 32],
            zk_attested: false,
            zk_program_hash: [0u8; 32],
            zk_capsule_commitment: [0u8; 32],
            policy: AttestPolicy {
                kernel_root: [0u8; 32],
                boot_epoch: 0,
                depth: 0,
                checked: 0,
                approval_present: 0,
                reserved: [0u8; 5],
                approval: [0u8; 128],
            },
        }
    }
}
