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

// One descriptor per packaged file: a NUL-padded absolute vfs path, then the
// absolute byte extent of its payload on the device. Every extent is bounded
// to the store's payload window and against a total allocation budget before
// any of it is read, so a hostile table cannot exhaust the capsule heap or
// send a read outside the store.
//
// Each descriptor stands or falls alone. The table carries no check of its
// own beyond the header, so one damaged descriptor says nothing about its
// neighbours: it is refused and counted, and the rest of the store still
// loads. Failing the whole table for one entry is how a single flipped bit
// used to empty /capsules and lose every record the machine keeps.
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Deref;

use super::error::BlkError;
use super::store_header::{le_u64, ENTRY_LEN, HEADER_LEN};
use super::wire::SECTOR_SIZE;

pub(super) use nonos_disk_map::{streamed, valid_name, MAX_TOTAL_BYTES, NAME_LEN, STREAMED_MAX_BYTES};

#[derive(Clone)]
pub struct TocEntry {
    pub name: String,
    pub offset: u64,
    pub len: u64,
    pub digest: [u8; 16],
    /// The descriptor's place in the table, which a write patches.
    pub slot: usize,
}

/// Where payloads may lie, in bytes from the start of the disk: from the
/// store's first byte, past the table the header's count needs, up to
/// `end`. An extent anywhere else names the GPT, the table itself or the
/// disk plan, and the kernel would refuse the read as if the disk had.
#[derive(Clone, Copy, Debug)]
pub struct Window {
    pub base: u64,
    pub end: u64,
}

/// The descriptors that stood up, in table order, and how many did not.
pub struct Toc {
    pub entries: Vec<TocEntry>,
    pub refused: usize,
}

impl Deref for Toc {
    type Target = [TocEntry];

    fn deref(&self) -> &[TocEntry] {
        &self.entries
    }
}

/// The table of `count` descriptors at the front of `toc`. Only a table
/// shorter than its count fails as a whole; the header was checked by
/// `entry_count` before the count was trusted to size anything.
pub fn decode(toc: &[u8], count: usize, window: Window) -> Result<Toc, BlkError> {
    let table = ENTRY_LEN
        .checked_mul(count)
        .and_then(|t| t.checked_add(HEADER_LEN))
        .ok_or(BlkError::BadContainer)?;
    if toc.len() < table {
        return Err(BlkError::BadContainer);
    }
    let span = table.div_ceil(SECTOR_SIZE) as u64 * SECTOR_SIZE as u64;
    let floor = window.base.checked_add(span).ok_or(BlkError::BadContainer)?;
    let mut entries = Vec::with_capacity(count);
    /*
     * Two budgets: what vfs loads into its heap, and what it streams from the
     * device and never holds (nonos_disk_map::streamed).
     */
    let mut loaded = MAX_TOTAL_BYTES;
    let mut streams = STREAMED_MAX_BYTES;
    for slot in 0..count {
        let at = HEADER_LEN + ENTRY_LEN * slot;
        let raw = &toc[at..at + ENTRY_LEN];
        let Some(entry) = descriptor(raw, slot, floor, window.end) else {
            say_refused(&alloc::format!("slot {slot}"), "its name or extent is out of bounds");
            continue;
        };
        let budget = if streamed(&entry.name) { &mut streams } else { &mut loaded };
        if entry.len <= *budget {
            *budget -= entry.len;
            entries.push(entry);
        } else {
            say_refused(&entry.name, "it is past the store's byte budget");
        }
    }
    Ok(Toc { refused: count - entries.len(), entries })
}

/// The bytes a payload named `name` may still take: what is left of the
/// budget `decode` counts its entry against at boot, loaded or streamed,
/// after `entries`. A write that checks the sum of every entry instead
/// counts the streamed collection against the loaded budget, and on a
/// standard image, 56 MiB loaded beside a 12 MiB collection, kept nothing;
/// one that passes this is one the next boot loads.
pub fn room_for(entries: &[TocEntry], name: &str) -> u64 {
    let stream = streamed(name);
    let budget = if stream { STREAMED_MAX_BYTES } else { MAX_TOTAL_BYTES };
    let spent: u64 = entries.iter().filter(|e| streamed(&e.name) == stream).map(|e| e.len).sum();
    budget.saturating_sub(spent)
}

/// One refused entry on the serial console, by name and why: a store with
/// one entry refused reads as status 9, and the name says which.
fn say_refused(what: &str, why: &str) {
    let line = alloc::format!("[VFS] store entry {what} refused: {why}");
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

/// One descriptor, or `None` when any field of it is out of bounds: an
/// extent that wraps, is not on a sector or lies outside `floor..end`, or a
/// name `valid_name` refuses. Its budget is the caller's to check.
fn descriptor(raw: &[u8], slot: usize, floor: u64, end: u64) -> Option<TocEntry> {
    let offset = le_u64(raw, NAME_LEN);
    let len = le_u64(raw, NAME_LEN + 8);
    let stop = offset.checked_add(len)?;
    if !offset.is_multiple_of(SECTOR_SIZE as u64) || offset < floor || stop > end {
        return None;
    }
    let name = decode_name(&raw[..NAME_LEN])?;
    let mut digest = [0u8; 16];
    digest.copy_from_slice(&raw[NAME_LEN + 16..NAME_LEN + 32]);
    Some(TocEntry { name, offset, len, digest, slot })
}

fn decode_name(field: &[u8]) -> Option<String> {
    let end = field.iter().position(|&b| b == 0).unwrap_or(field.len());
    let name = core::str::from_utf8(&field[..end]).ok()?;
    valid_name(name).then(|| String::from(name))
}
