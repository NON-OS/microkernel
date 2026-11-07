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

//! The chunk list: the image's length, its chunk size and each chunk's
//! BLAKE3. The list's own BLAKE3 is the image's measurement, so a list is
//! taken only when it hashes to the pin, and a chunk only when it hashes to
//! its row.

use core::ops::Range;

use crate::format::{u32_at, u64_at, zero, DIGEST_LEN, LIST_HEAD, LIST_MAGIC, VERSION};
use crate::format::{CHUNK_MAX, CHUNK_MIN, MAX_IMAGE};
use crate::refused::Refused;

#[derive(Clone, Copy, Debug)]
pub struct ChunkList<'a> {
    pub total: u64,
    pub chunk: u32,
    digests: &'a [u8],
}

impl<'a> ChunkList<'a> {
    /// `b` as a chunk list whose BLAKE3 is `pin`.
    pub fn parse(b: &'a [u8], pin: &[u8; 32]) -> Result<Self, Refused> {
        if blake3::hash(b).as_bytes() != pin {
            return Err(Refused::Digest);
        }
        Self::read(b)
    }

    /// `b` as a chunk list, held to the layout alone. `parse` is the one an
    /// install uses; this one is for a list whose pin is checked elsewhere.
    pub fn read(b: &'a [u8]) -> Result<Self, Refused> {
        if b.len() < LIST_HEAD || b[..8] != LIST_MAGIC {
            return Err(Refused::Magic);
        }
        let version = u32_at(b, 8);
        if version != u32::from(VERSION) || !zero(b, 24, LIST_HEAD) {
            return Err(Refused::Magic);
        }
        let (chunk, total) = (u32_at(b, 12), u64_at(b, 16));
        if !chunk.is_power_of_two() || !(CHUNK_MIN..=CHUNK_MAX).contains(&chunk) {
            return Err(Refused::TooLarge);
        }
        if total == 0 || total > MAX_IMAGE {
            return Err(Refused::TooLarge);
        }
        let count = total.div_ceil(u64::from(chunk)) as usize;
        if b.len() != LIST_HEAD + count * DIGEST_LEN {
            return Err(Refused::Layout);
        }
        Ok(ChunkList { total, chunk, digests: &b[LIST_HEAD..] })
    }

    pub fn count(&self) -> usize {
        self.digests.len() / DIGEST_LEN
    }

    /// The bytes of the image chunk `i` covers.
    pub fn range(&self, i: usize) -> Option<Range<u64>> {
        let from = (i as u64).checked_mul(u64::from(self.chunk))?;
        (i < self.count()).then(|| from..(from + u64::from(self.chunk)).min(self.total))
    }

    /// Whether `bytes` are chunk `i`, whole.
    pub fn holds(&self, i: usize, bytes: &[u8]) -> bool {
        let Some(r) = self.range(i) else { return false };
        let want = &self.digests[i * DIGEST_LEN..(i + 1) * DIGEST_LEN];
        bytes.len() as u64 == r.end - r.start && blake3::hash(bytes).as_bytes()[..] == *want
    }
}
