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

//! Store entries served from the device, read by read, never held.
//!
//! Everything else in the package store is loaded into memory when it is
//! staged. A streamed entry (`nonos_disk_map::streamed`) is not: the
//! wallpaper collection is a dozen megabytes, and a session shows only the
//! few its owner kept at setup. Its place on the device is kept instead, and
//! each read is a read of the device. Its reader checks what it reads: the
//! catalog holds each wallpaper to a SHA-256 pinned in its own signed image,
//! so the table's digest, which covers the whole entry, is not used here.

use alloc::vec;
use alloc::vec::Vec;

use super::client::read_blocks;
use super::error::BlkError;
use super::store::sector_span;
use super::wire::{MAX_READ_BYTES, SECTOR_SIZE};

/// Where a streamed entry's bytes are on the device.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Extent {
    pub offset: u64,
    pub len: u64,
}

/// Up to `max` bytes of the entry from `at`, no more than one device
/// request's worth: the caller reads again for the rest, as a reader of any
/// file does.
pub fn read_range(e: &Extent, at: u64, max: usize) -> Result<Vec<u8>, BlkError> {
    if at >= e.len || max == 0 {
        return Ok(Vec::new());
    }
    let want = core::cmp::min(e.len - at, max as u64) as usize;
    let start = e.offset + at;
    let skip = (start % SECTOR_SIZE as u64) as usize;
    let want = core::cmp::min(want, MAX_READ_BYTES - SECTOR_SIZE);
    let mut scratch = vec![0u8; sector_span(skip + want)];
    read_blocks(start / SECTOR_SIZE as u64, &mut scratch)?;
    Ok(scratch[skip..skip + want].to_vec())
}
