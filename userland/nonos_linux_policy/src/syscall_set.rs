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

//! Which system calls a program may complete: one bit per bucket of the
//! kernel's seal, so the set is the proof's A exactly. Bucket b is
//! call number b below 512; every larger number shares bucket 512.

use crate::error::PolicyError;

pub const BUCKETS: usize = 513;
/// 513 bits, rounded up to bytes.
pub const SET_LEN: usize = 65;
/// The last byte carries only bucket 512, in its low bit.
const LAST_BYTE_BITS: u8 = 0x01;

/// The bucket of call number `nr`, as the kernel counts it.
pub const fn bucket(nr: u64) -> usize {
    if nr < 512 {
        nr as usize
    } else {
        512
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyscallSet([u8; SET_LEN]);

impl SyscallSet {
    pub const NONE: Self = Self([0; SET_LEN]);
    pub const ALL: Self = {
        let mut b = [0xFF; SET_LEN];
        b[SET_LEN - 1] = LAST_BYTE_BITS;
        Self(b)
    };

    /// Refuses a bit past bucket 512, which no call can land in.
    pub fn from_bytes(b: [u8; SET_LEN]) -> Result<Self, PolicyError> {
        if b[SET_LEN - 1] & !LAST_BYTE_BITS != 0 {
            return Err(PolicyError::SyscallSet);
        }
        Ok(Self(b))
    }

    pub fn as_bytes(&self) -> &[u8; SET_LEN] {
        &self.0
    }

    /// Whether call number `nr` may complete.
    pub fn allows(&self, nr: u64) -> bool {
        let b = bucket(nr);
        self.0[b / 8] & (1 << (b % 8)) != 0
    }

    /// The set with call number `nr`'s bucket added.
    pub fn with(mut self, nr: u64) -> Self {
        let b = bucket(nr);
        self.0[b / 8] |= 1 << (b % 8);
        self
    }
}
