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

//! How many bytes of a buffer a commit copies. Pure, so the host proofs
//! hold it.
//!
//! The frame is this capsule's own copy of the client's pixels, kept in its
//! heap beside the program it loaded. A pool is bounded only by what the
//! client has mapped, and a PROT_NONE reservation of any size costs a guest
//! nothing, so a buffer could claim gigabytes and a commit asked this
//! capsule for all of them at once, which ended it and every process of the
//! family. MAX_FRAME is the largest frame shown: more than a 2560x1600
//! window at four bytes a pixel, above the 1920x1080 screen a guest's
//! window is placed on, and a quarter of the 64 MiB the kernel registers.

/// The most bytes one frame may take.
pub const MAX_FRAME: u64 = 16 << 20;

/// The bytes of a `stride` by `height` buffer at `offset` of a pool of
/// `pool` bytes; None when it is empty, runs past the pool, or is larger
/// than MAX_FRAME.
pub fn frame_len(offset: u64, stride: u32, height: u32, pool: u64) -> Option<usize> {
    let bytes = u64::from(stride).checked_mul(u64::from(height))?;
    if bytes == 0 || bytes > MAX_FRAME || offset.checked_add(bytes)? > pool {
        return None;
    }
    usize::try_from(bytes).ok()
}
