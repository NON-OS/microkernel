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

//! Reading the frames out of a batch body.

extern crate alloc;

use alloc::vec::Vec;

/// The frames in a batch body, `[u32 count]` then `[u32 len][frame]` per
/// frame. `None` for a body that does not hold what its count says: a
/// batch is taken whole or not at all, never partly and silently.
pub fn batch_frames(body: &[u8]) -> Option<Vec<Vec<u8>>> {
    let count = u32::from_le_bytes(body.get(..4)?.try_into().ok()?) as usize;
    let mut frames = Vec::with_capacity(count.min(64));
    let mut at = 4;
    for _ in 0..count {
        let len = u32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?) as usize;
        frames.push(body.get(at + 4..at + 4 + len)?.to_vec());
        at += 4 + len;
    }
    (at == body.len()).then_some(frames)
}
