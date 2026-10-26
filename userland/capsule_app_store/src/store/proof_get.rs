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

//! The container, put together from GETs: in order, from 0 to exactly the
//! total STATUS named. A chunk that is empty, over GET_MAX or past the total
//! is refused, and so is a total of nothing or over the container's 1 MiB.
//! The limits are nonos_proof_abi's, held to it by proof_call.rs.
//!
//! Pure; market_proofs holds it.

use alloc::vec::Vec;

/// wire/prove.rs's GET_MAX.
pub const GET_MAX: usize = 64 * 1024;
/// container/parse.rs's MAX_BYTES.
pub const MAX_BYTES: u32 = 1 << 20;

/// A total a container can have.
pub fn fits(total: u32) -> bool {
    total != 0 && total <= MAX_BYTES
}

/// Where the next GET starts after `len` bytes at `at`, or None for a
/// chunk this container cannot take.
pub fn next(at: u32, total: u32, len: usize) -> Option<u32> {
    if len == 0 || len > GET_MAX || !fits(total) {
        return None;
    }
    let end = at.checked_add(u32::try_from(len).ok()?)?;
    (end <= total).then_some(end)
}

pub struct Assembly {
    total: u32,
    /// What has come, wiped by its holder (proof.rs) when dropped.
    pub bytes: Vec<u8>,
}

impl Assembly {
    pub fn new(total: u32) -> Option<Assembly> {
        fits(total).then(|| Assembly { total, bytes: Vec::new() })
    }

    /// Where the next GET starts.
    pub fn at(&self) -> u32 {
        self.bytes.len() as u32
    }

    /// Takes the chunk the GET at `at()` answered: false, and nothing
    /// taken, for one `next` refuses.
    pub fn take(&mut self, chunk: &[u8]) -> bool {
        if next(self.at(), self.total, chunk.len()).is_none() {
            return false;
        }
        self.bytes.extend_from_slice(chunk);
        true
    }

    pub fn done(&self) -> bool {
        self.at() == self.total
    }
}
