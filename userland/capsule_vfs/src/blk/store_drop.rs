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

//! The sector writes that drop one descriptor from the table, in the order
//! they must land, so the power can fail between any two of them.
//!
//! Shifting every later descriptor down a slot, as the remover did, rewrites
//! many sectors, and a descriptor whose name and extent sit in different
//! sectors then had one half shifted and the other not: one file's name over
//! another file's extent and digest, which verifies. So nothing shifts. The
//! last descriptor is copied over the dropped one and the count comes down
//! by one, header last; the slot past the new count is left as it is, and
//! an append clears it before reuse.
//!
//! Most descriptors lie in one sector, and the copy is one write: until it
//! lands the old table stands, after it the last file is named twice, the
//! same bytes both times, until the header lands. A descriptor at a slot
//! that is 3 mod 4 has its name at the end of one sector and its extent at
//! the start of the next. Its extent is first made one no decode accepts,
//! then its name written, then its real extent: in between, that one slot
//! is refused and reported, never a name over the wrong bytes.
//!
//! Pure, so the order can be driven on the host against a real table.

use alloc::vec::Vec;

use super::error::BlkError;
use super::store_header::{ENTRY_LEN, HEADER_LEN};
use super::store_toc::NAME_LEN;
use super::wire::SECTOR_SIZE;

/// An offset no descriptor can carry: not on a sector.
const UNLOADABLE: u64 = 1;

/// The writes, as (sector of the table, its new bytes), that drop slot
/// `index` from `toc`, a table of `count` descriptors read whole.
pub fn drop_slot(
    toc: &[u8],
    count: usize,
    index: usize,
) -> Result<Vec<(usize, [u8; SECTOR_SIZE])>, BlkError> {
    let table = ENTRY_LEN
        .checked_mul(count)
        .and_then(|t| t.checked_add(HEADER_LEN))
        .ok_or(BlkError::BadContainer)?;
    if index >= count || toc.len() < table || !toc.len().is_multiple_of(SECTOR_SIZE) {
        return Err(BlkError::BadContainer);
    }
    let mut t = toc.to_vec();
    let mut writes = Vec::new();
    let last = count - 1;
    if index != last {
        let from = HEADER_LEN + ENTRY_LEN * last;
        let to = HEADER_LEN + ENTRY_LEN * index;
        let mut src = [0u8; ENTRY_LEN];
        src.copy_from_slice(&t[from..from + ENTRY_LEN]);
        let (name_sector, tail_sector) = (to / SECTOR_SIZE, (to + ENTRY_LEN - 1) / SECTOR_SIZE);
        if name_sector == tail_sector {
            t[to..to + ENTRY_LEN].copy_from_slice(&src);
            if name_sector != 0 {
                writes.push(sector(&t, name_sector));
            }
        } else {
            let tail = to + NAME_LEN;
            t[tail..tail + 8].copy_from_slice(&UNLOADABLE.to_le_bytes());
            writes.push(sector(&t, tail_sector));
            t[to..tail].copy_from_slice(&src[..NAME_LEN]);
            writes.push(sector(&t, name_sector));
            t[tail..to + ENTRY_LEN].copy_from_slice(&src[NAME_LEN..]);
            writes.push(sector(&t, tail_sector));
        }
    }
    t[12..16].copy_from_slice(&(last as u32).to_le_bytes());
    writes.push(sector(&t, 0));
    Ok(writes)
}

fn sector(t: &[u8], n: usize) -> (usize, [u8; SECTOR_SIZE]) {
    let mut s = [0u8; SECTOR_SIZE];
    s.copy_from_slice(&t[n * SECTOR_SIZE..(n + 1) * SECTOR_SIZE]);
    (n, s)
}
