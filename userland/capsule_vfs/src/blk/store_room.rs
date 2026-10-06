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

//! Room for the table to grow, and for the payload an append brings.
//!
//! The installer and vfs put payloads past a table of `MAX_ENTRIES`, so a
//! table that grows meets zeros. `tools/nonos-store-pack`, which packs the
//! live USB stick and the QEMU disk, puts the first payload right after the
//! table its entries need; every fourth entry the table needs one more
//! sector, and the appender used to write that sector over the payload.
//!
//! A payload in the way is copied to a free extent, where nothing points at
//! it, and its descriptor is then repointed in one sector write: offset,
//! length and digest share a sector. Before that write the old copy is the
//! live one, after it the new one is, and the length and digest describe
//! both. Only then does the table grow over the space it left.
//!
//! Everything is placed before anything is written, so a store with no
//! room refuses the append whole rather than after moving half of it.

use alloc::vec;
use alloc::vec::Vec;

use super::client::read_blocks;
use super::error::BlkError;
use super::store_entry::patch_entry;
use super::store_free::free_extent;
use super::store_toc::TocEntry;
use super::store_write::{commit_entry, write_sectors, MAX_WRITE_BYTES};
use super::wire::SECTOR_SIZE;

/// Where it all goes: each payload that starts below the grown table's end,
/// by its place in `entries`, and then the new payload.
pub struct Placement {
    pub moves: Vec<(usize, u64)>,
    pub at: u64,
}

/// Place, without writing, every payload in the way of a table that will
/// end at `table_end`, then a new payload of `len` bytes, each in the first
/// free extent at or past `floor` as the ones placed before it leave the
/// store. `NoSpace` when any of them would pass `end`.
pub fn place(
    entries: &[TocEntry],
    table_end: u64,
    floor: u64,
    end: u64,
    len: u64,
) -> Result<Placement, BlkError> {
    let mut after = entries.to_vec();
    let mut moves = Vec::new();
    for i in 0..after.len() {
        if after[i].offset >= table_end {
            continue;
        }
        let to = within(free_extent(&after, floor, after[i].len), after[i].len, end)?;
        moves.push((i, to));
        after[i].offset = to;
    }
    let at = within(free_extent(&after, floor, len), len, end)?;
    Ok(Placement { moves, at })
}

fn within(at: u64, len: u64, end: u64) -> Result<u64, BlkError> {
    match at.checked_add(len) {
        Some(stop) if stop <= end => Ok(at),
        _ => Err(BlkError::NoSpace),
    }
}

/// Carry out `moves`: copy each payload, then repoint its descriptor. `toc`
/// and `entries` follow, so the table the append commits names the copies.
pub fn make_room(
    toc: &mut Vec<u8>,
    entries: &mut [TocEntry],
    moves: &[(usize, u64)],
) -> Result<(), BlkError> {
    for &(i, to) in moves {
        let entry = entries.get_mut(i).ok_or(BlkError::BadContainer)?;
        copy_extent(entry.offset, to, entry.len)?;
        let region = patch_entry(toc, entry.slot, to, &entry.digest)?;
        commit_entry(&region, entry.slot)?;
        *toc = region;
        entry.offset = to;
    }
    Ok(())
}

/// `len` bytes from `from` to `to`, whole sectors at a time. The two never
/// overlap: `to` was placed clear of every live extent, `from`'s included.
fn copy_extent(from: u64, to: u64, len: u64) -> Result<(), BlkError> {
    let sector = SECTOR_SIZE as u64;
    let mut done = 0u64;
    while done < len {
        let take = (len - done).min(MAX_WRITE_BYTES as u64) as usize;
        let mut buf = vec![0u8; take.div_ceil(SECTOR_SIZE) * SECTOR_SIZE];
        read_blocks((from + done) / sector, &mut buf)?;
        write_sectors((to + done) / sector, &buf)?;
        done += take as u64;
    }
    Ok(())
}
