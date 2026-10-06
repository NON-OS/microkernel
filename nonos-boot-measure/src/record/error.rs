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

/// Why a boot-root record was refused. Each has a stable code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordError {
    /// Not exactly `RECORD_LEN` bytes.
    Length,
    /// A root word at or above the field modulus.
    NonCanonicalRoot,
    /// The signature does not verify under the release key.
    BadSignature,
    /// The epoch is below the TPM's anti-rollback floor: a replayed record.
    Stale,
}

impl RecordError {
    pub const fn code(self) -> u32 {
        match self {
            RecordError::Length => 1,
            RecordError::NonCanonicalRoot => 2,
            RecordError::BadSignature => 3,
            RecordError::Stale => 4,
        }
    }
}
