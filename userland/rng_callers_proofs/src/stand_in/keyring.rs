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

//! The keyring capsule's client, the TPM key's error, the audit log and the
//! scrub routine, as the mounted files name them. The keyring keeps, per
//! thread, each key it was handed.

use std::cell::RefCell;

#[path = "../../../../src/security/keyring_capsule/error.rs"]
mod keyring_error;
#[path = "../../../../src/security/keyring_capsule/types.rs"]
mod keyring_types;

thread_local! {
    static STORED: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

/// Every key handed to the keyring on this thread.
pub fn stored() -> Vec<Vec<u8>> {
    STORED.with_borrow(Clone::clone)
}

pub mod keyring_capsule {
    pub use super::keyring_error::KeyringCapsuleError;
    pub use super::keyring_types::KeyType;

    pub fn store(_t: KeyType, data: &[u8], _expires: u64) -> Result<u32, KeyringCapsuleError> {
        super::STORED.with_borrow_mut(|s| {
            s.push(data.to_vec());
            Ok(s.len() as u32)
        })
    }
}

pub mod machine_key {
    /// Only named by the volume's error type here.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum KeyError {
        NoRandomness,
    }
}

pub mod audit {
    pub enum AuditSeverity {
        Emergency,
    }
    pub fn log_security_violation(description: String, _severity: AuditSeverity) {
        crate::stand_in::kernel::keep(description);
    }
}

pub mod scrub_asm {
    /// The scrub routine's contract: zero exactly `len` bytes at `ptr`.
    ///
    /// # Safety
    /// `ptr` must be valid for writes of `len` bytes.
    pub unsafe fn nonos_scrub_bytes(ptr: *mut u8, len: usize) {
        // SAFETY: the caller's contract, passed on.
        unsafe { core::ptr::write_bytes(ptr, 0, len) };
    }
}
