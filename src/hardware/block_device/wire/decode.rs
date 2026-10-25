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

//! The GEOMETRY reply's payload. Pure, so kernel_proofs runs it on the host.

use super::header::GEOMETRY_LEN;

const REMOVABLE: u32 = 1 << 0;
const READ_ONLY: u32 = 1 << 1;
const MEDIA_PRESENT: u32 = 1 << 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Geometry {
    pub(crate) lba_count: u64,
    pub(crate) lba_bytes: u32,
    pub(crate) max_transfer_bytes: u32,
    pub(crate) flags: u32,
}

impl Geometry {
    pub(crate) fn removable(&self) -> bool {
        self.flags & REMOVABLE != 0
    }

    pub(crate) fn read_only(&self) -> bool {
        self.flags & READ_ONLY != 0
    }

    pub(crate) fn media_present(&self) -> bool {
        self.flags & MEDIA_PRESENT != 0
    }
}

/// None for a payload shorter than the wire's twenty bytes; anything after
/// them is a later version's and is left alone.
pub(crate) fn decode(body: &[u8]) -> Option<Geometry> {
    let b = body.get(..GEOMETRY_LEN)?;
    let u32_at = |at: usize| u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]);
    let mut count = [0u8; 8];
    count.copy_from_slice(&b[..8]);
    Some(Geometry {
        lba_count: u64::from_le_bytes(count),
        lba_bytes: u32_at(8),
        max_transfer_bytes: u32_at(12),
        flags: u32_at(16),
    })
}
