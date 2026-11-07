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

//! Which chunks hold a stretch of the image, and that stretch cut out of
//! them, so a member is read without the whole image in memory.

use alloc::vec::Vec;
use core::ops::Range;

use crate::chunks::ChunkList;

/// The chunks that hold `at`, a stretch of the image; empty for an empty or
/// out-of-image stretch.
pub fn covering(list: &ChunkList, at: &Range<u64>) -> Range<usize> {
    if at.start >= at.end || at.end > list.total {
        return 0..0;
    }
    let c = u64::from(list.chunk);
    (at.start / c) as usize..at.end.div_ceil(c) as usize
}

impl ChunkList<'_> {
    /// `at` cut out of `chunks`, which are `covering(at)`'s chunks in order,
    /// each already held to its digest. None when they do not cover it.
    pub fn cut(&self, at: &Range<u64>, chunks: &[&[u8]]) -> Option<Vec<u8>> {
        let span = covering(self, at);
        if span.is_empty() || chunks.len() != span.len() {
            return None;
        }
        let first = self.range(span.start)?.start;
        let mut out = Vec::with_capacity((at.end - at.start) as usize);
        for c in chunks {
            out.extend_from_slice(c);
        }
        let from = (at.start - first) as usize;
        let to = (at.end - first) as usize;
        out.get(from..to).map(<[u8]>::to_vec)
    }
}
