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

//! The window one `MkInstallSource` call reads: bounded, and checked at both
//! ends against the directmap rather than at its base alone, so a region the
//! loader recorded as straddling the map's edge cannot be read past it on
//! the last chunk.

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL};
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::unified::phys_to_virt;

/// One transfer at most; the caller loops. Bounds the kernel-side copy so a
/// single call cannot hold the CPU for a hundred megabytes.
const MAX_CHUNK: u64 = 1 << 20;

/// The kernel-virtual start of the chunk and its length, zero at the end of
/// the image; an errno when the request or the region is out of bounds.
pub fn window(base: u64, size: u64, offset: u64, out_len: u64) -> Result<(VirtAddr, u64), i64> {
    if offset > size {
        return Err(ERRNO_INVAL);
    }
    let want = (size - offset).min(out_len).min(MAX_CHUNK);
    if want == 0 {
        return Ok((VirtAddr::zero(), 0));
    }
    let start = base.checked_add(offset).ok_or(ERRNO_FAULT)?;
    let last = start.checked_add(want - 1).ok_or(ERRNO_FAULT)?;
    phys_to_virt(PhysAddr::new(last)).ok_or(ERRNO_FAULT)?;
    let virt = phys_to_virt(PhysAddr::new(start)).ok_or(ERRNO_FAULT)?;
    Ok((virt, want))
}
