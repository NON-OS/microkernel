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

//! A token with given caps, and the calls the gate is checked over.

use crate::capabilities::{Capability, CapabilityToken};
use crate::syscall::abi::REGISTRY;
use crate::syscall::numbers::SyscallNumber;

extern crate alloc;
use alloc::vec::Vec;

// A token with a given set of capabilities and no expiry.
pub(crate) fn token(permissions: Vec<Capability>) -> CapabilityToken {
    CapabilityToken {
        owner_module: 0,
        permissions,
        expires_at_ms: None,
        nonce: 0,
        signature: [0u8; 64],
        token_id: 0,
        subject_capsule_id: 0,
        subject_asid: 0,
        subject_measurement: [0u8; 32],
        boot_session_nonce: [0u8; 16],
        revocation_epoch: 0,
        delegation_depth: 0,
    }
}

// Every call the ABI registry holds. Ids are four-letter tags, so a scan
// of small numbers would find none of them.
pub(super) fn registered_syscalls() -> Vec<SyscallNumber> {
    REGISTRY.iter().flat_map(|d| d.iter().map(|e| e.variant)).collect()
}
