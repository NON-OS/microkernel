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

//! Appends a named payload to the on-disk NONOSTR1 store, committing sector 0 last.
//!
//! In the order it lands: any payload in the way of the grown table moves
//! (`store_room`), the new payload goes to a free extent nothing points at,
//! the table's other sectors follow, and sector 0 with the new count is
//! last. A write cut short anywhere leaves the store as it was or as it
//! will be, never a descriptor naming bytes that did not land.
use alloc::vec;

use nonos_disk_map::{digest16, STORE_BASE_LBA, TOC_SPAN};
use nonos_libc::mk_store_write;

use super::client::{capacity, read_blocks, read_span};
use super::error::BlkError;
use super::store_header::{entry_count, ENTRY_LEN, HEADER_LEN, MAX_ENTRIES};
use super::store_room::{make_room, place};
use super::store_toc::{decode, room_for, valid_name, TocEntry, Window, NAME_LEN};
use super::wire::SECTOR_SIZE;

/// The kernel's store write takes at most this much a call.
pub(super) const MAX_WRITE_BYTES: usize = 8192;

pub fn append(name: &str, data: &[u8]) -> Result<(), BlkError> {
    let mut head = [0u8; SECTOR_SIZE];
    read_blocks(STORE_BASE_LBA, &mut head)?;
    let count = entry_count(&head)?;
    let mut toc = vec![0u8; sector_span(HEADER_LEN + ENTRY_LEN * count)];
    read_span(STORE_BASE_LBA, &mut toc)?;
    let capacity_bytes = capacity()?.checked_mul(SECTOR_SIZE as u64).ok_or(BlkError::BadLength)?;
    let base = STORE_BASE_LBA * SECTOR_SIZE as u64;
    /*
     * Payloads go past a full table, so nothing this writes is ever in the
     * way of the table growing again.
     */
    let floor = base + TOC_SPAN as u64;
    let mut entries = decode(&toc, count, Window { base, end: capacity_bytes })?;
    if let Some(index) = entries.iter().position(|e| e.name == name) {
        return same_bytes(&entries[index], data).or_else(|_| {
            super::store_replace::replace(&toc, index, &entries, floor, capacity_bytes, data)
        });
    }
    if data.len() as u64 > room_for(&entries, name) || count >= MAX_ENTRIES {
        return Err(BlkError::NoSpace);
    }
    if !valid_name(name) {
        return Err(BlkError::Inval);
    }
    let region_len = sector_span(HEADER_LEN + ENTRY_LEN * (count + 1));
    let len = data.len() as u64;
    let plan = place(&entries, base + region_len as u64, floor, capacity_bytes, len)?;
    make_room(&mut toc, &mut entries.entries, &plan.moves)?;
    write_payload(plan.at, data)?;
    let mut region = vec![0u8; region_len];
    region[..toc.len()].copy_from_slice(&toc);
    /*
     * The slot past the count may still hold a removed descriptor; none of
     * it may show through behind a shorter name.
     */
    let slot = HEADER_LEN + ENTRY_LEN * count;
    let e = &mut region[slot..slot + ENTRY_LEN];
    e.fill(0);
    e[..name.len()].copy_from_slice(name.as_bytes());
    e[NAME_LEN..NAME_LEN + 8].copy_from_slice(&plan.at.to_le_bytes());
    e[NAME_LEN + 8..NAME_LEN + 16].copy_from_slice(&len.to_le_bytes());
    e[NAME_LEN + 16..ENTRY_LEN].copy_from_slice(&digest16(data));
    region[12..16].copy_from_slice(&(count as u32 + 1).to_le_bytes());
    commit(&region)
}

/*
 * Re-persisting the exact bytes already committed is the idempotent retry the
 * store_persist path relies on, so it succeeds without touching the disk.
 * Anything else goes to `store_replace`, which overwrites in place when the
 * length is unchanged and refuses when it is not: this appender cannot move an
 * extent, and reporting success there would leave the caller believing the old
 * bytes had been replaced.
 */
fn same_bytes(entry: &TocEntry, data: &[u8]) -> Result<(), BlkError> {
    if entry.len == data.len() as u64 && entry.digest == digest16(data) {
        return Ok(());
    }
    Err(BlkError::Exists)
}

pub(super) fn write_payload(next_off: u64, data: &[u8]) -> Result<(), BlkError> {
    let mut lba = next_off / SECTOR_SIZE as u64;
    let mut done = 0usize;
    while done < data.len() {
        let take = core::cmp::min(MAX_WRITE_BYTES, data.len() - done);
        let mut buf = vec![0u8; sector_span(take)];
        buf[..take].copy_from_slice(&data[done..done + take]);
        write_sectors(lba, &buf)?;
        lba += (buf.len() / SECTOR_SIZE) as u64;
        done += take;
    }
    Ok(())
}

/// The one sector of the table that holds entry `index`'s offset and digest.
pub(super) fn commit_entry(region: &[u8], index: usize) -> Result<(), BlkError> {
    let sector = super::store_entry::entry_sector(index);
    let at = sector * SECTOR_SIZE;
    write_sectors(STORE_BASE_LBA + sector as u64, &region[at..at + SECTOR_SIZE])
}

pub(super) fn commit(region: &[u8]) -> Result<(), BlkError> {
    let mut off = SECTOR_SIZE;
    while off < region.len() {
        let take = core::cmp::min(MAX_WRITE_BYTES, region.len() - off);
        let lba = STORE_BASE_LBA + (off / SECTOR_SIZE) as u64;
        write_sectors(lba, &region[off..off + take])?;
        off += take;
    }
    write_sectors(STORE_BASE_LBA, &region[..SECTOR_SIZE])
}

/// No disk the kernel drives carries NONOS. The store may still have been
/// read, from the loader's copy, so a keep is the first to hear of it.
const ENODEV: i64 = -19;

/// A write the device took only part of is a failed write: the caller
/// must not go on to commit a table naming sectors that never landed.
pub(super) fn write_sectors(lba: u64, buf: &[u8]) -> Result<(), BlkError> {
    match mk_store_write(lba, buf.as_ptr(), buf.len()) {
        rc if rc == buf.len() as i64 => Ok(()),
        ENODEV => Err(BlkError::NoService),
        rc => Err(BlkError::Transport(rc)),
    }
}

fn sector_span(bytes: usize) -> usize {
    bytes.div_ceil(SECTOR_SIZE) * SECTOR_SIZE
}
