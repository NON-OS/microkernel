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

//! One range of the disk, read into loader-data pages.

use uefi::prelude::*;
use uefi::proto::media::block::BlockIO;

use super::super::pages::{Pages, PAGE};

/* Bytes per firmware read, as the store copy reads. */
const CHUNK: usize = 256 * 1024;

/// Sectors `lba..lba + sectors` (512 bytes each) of the disk behind `blk`,
/// whose blocks are `block` bytes, in pages the caller keeps or drops.
/// `None` when the range does not start on a block or a read fails.
pub(super) fn read_range<'a>(
    bs: &'a BootServices,
    blk: &BlockIO,
    block: u64,
    lba: u64,
    sectors: u64,
) -> Option<Pages<'a>> {
    let at = lba.checked_mul(512)?;
    if at % block != 0 {
        return None;
    }
    /* Whole blocks, so the last one may run past the file: it is the disk's. */
    let len = usize::try_from(sectors.checked_mul(512)?.div_ceil(block) * block).ok()?;
    let pages = Pages::new(bs, len.div_ceil(PAGE))?;
    let bytes = &mut pages.bytes()[..len];
    let mut done = 0usize;
    while done < len {
        let take = (len - done).min(CHUNK);
        let first = (at + done as u64) / block;
        blk.read_blocks(blk.media().media_id(), first, &mut bytes[done..done + take]).ok()?;
        done += take;
    }
    Some(pages)
}
