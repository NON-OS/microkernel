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

//! Which sectors the loader's copy of the store holds. Pure, so the
//! arithmetic is held on the host (kernel_proofs).

pub(super) const STORE_BASE_LBA: u64 = 256;
pub(super) const SECTOR: usize = 512;

/*
 * The loader hands over the length the table of contents names, which ends
 * where the last entry's bytes end, inside a sector. Every store read is of
 * whole sectors, so the copy is counted to the end of that sector: the loader
 * read whole pages from the disk, so those bytes are the disk's too. Counted
 * to the byte, the last entry's final read missed the copy and went to a disk
 * the kernel may not drive.
 */
pub(super) const fn held(len: usize) -> usize {
    len.div_ceil(SECTOR) * SECTOR
}

/// Where in a copy of `held` bytes sectors `lba..` of `len` bytes start;
/// `None` when the copy does not hold every one of them.
pub(super) fn offset(lba: u64, len: usize, held: usize) -> Option<usize> {
    let off = usize::try_from(lba.checked_sub(STORE_BASE_LBA)?).ok()?.checked_mul(SECTOR)?;
    if off.checked_add(len)? > held {
        return None;
    }
    Some(off)
}
