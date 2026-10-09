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

use super::types::CryptoVerifyResult;

impl CryptoVerifyResult {
    pub fn attest_policy(&self) -> crate::handoff::types::AttestPolicy {
        use crate::handoff::types::AttestPolicy;
        if self.path_attested {
            use super::self_attest::{enrolled_root, BOOT_EPOCH, DEPTH};
            return AttestPolicy {
                kernel_root: enrolled_root(),
                boot_epoch: BOOT_EPOCH,
                depth: DEPTH as u8,
                checked: 1,
                approval_present: 0,
                reserved: [0u8; 5],
                approval: [0u8; 128],
            };
        }
        AttestPolicy {
            kernel_root: [0u8; 32],
            boot_epoch: 0,
            depth: 0,
            checked: 0,
            approval_present: 0,
            reserved: [0u8; 5],
            approval: [0u8; 128],
        }
    }

    pub fn kernel_attested(&self) -> bool {
        self.path_attested
    }
}
