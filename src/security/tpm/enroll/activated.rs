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

//! The secret ActivateCredential gave back, in a buffer that wipes itself.

use super::consts::DIGEST_MAX;
use crate::security::hardening::memory_sanitization::secure_zero_slice;

/// What the registrar sealed to this TPM, at most a SHA-512 digest long. The
/// response it came out of is wiped by `run`; this copy is wiped on drop.
pub struct Activated {
    bytes: [u8; DIGEST_MAX],
    len: usize,
}

impl Activated {
    /// `None` for an empty or overlong secret, which no MakeCredential makes.
    pub(super) fn copy(src: &[u8]) -> Option<Self> {
        if src.is_empty() || src.len() > DIGEST_MAX {
            return None;
        }
        let mut out = Self { bytes: [0u8; DIGEST_MAX], len: src.len() };
        out.bytes.get_mut(..src.len())?.copy_from_slice(src);
        Some(out)
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self.bytes.get(..self.len) {
            Some(b) => b,
            None => &[],
        }
    }
}

impl Drop for Activated {
    fn drop(&mut self) {
        secure_zero_slice(&mut self.bytes);
    }
}
