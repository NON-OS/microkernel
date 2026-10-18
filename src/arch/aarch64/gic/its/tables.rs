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

//! The tables the ITS keeps in memory, one `GITS_BASER<n>` each: device
//! table, collection table, and on GICv4 parts vPE tables, which no LPI
//! needs and which stay invalid.

use super::encode::attrs::coherent;
use super::encode::baser::{
    entry_bytes, page_bytes, page_of, table, table_type, MAX_PAGES, PAGE_16K, PAGE_4K, PAGE_64K,
    TYPE_COLLECTIONS, TYPE_DEVICES,
};
use super::encode::typer::ItsTyper;
use super::memory::{alloc, free};
use super::regs::{Regs, BASER};

/// Program every table register, the device table holding `device_ids` ids
/// where the ITS's bits and `Size` allow. Returns how many it holds: an id
/// past that is refused before any command names it, since a MAPD for one
/// is a command error the queue then stalls on.
pub(super) fn program(regs: Regs, typer: &ItsTyper, device_ids: u64) -> Result<u64, &'static str> {
    let (mut devices, mut collections) = (None, typer.hcc != 0);
    for n in 0..8 {
        let at = BASER + n * 8;
        let read = regs.read64(at);
        match table_type(read) {
            TYPE_DEVICES => {
                let wanted = device_ids.min(1 << typer.device_bits);
                devices = Some(one(regs, at, read, wanted)?.min(1 << typer.device_bits));
            }
            // Collection 0 is all the kernel maps, so one entry will do.
            TYPE_COLLECTIONS if typer.hcc == 0 => collections = one(regs, at, read, 1)? != 0,
            _ => regs.write64(at, 0),
        }
    }
    if !collections {
        return Err("the ITS has no collection table and holds no collections itself");
    }
    devices.ok_or("the ITS has no device table")
}

/// A flat table of at least `entries` entries, or as many as `Size` allows,
/// in the first page size the ITS takes. Returns the entries it holds.
fn one(regs: Regs, at: u64, read: u64, entries: u64) -> Result<u64, &'static str> {
    let bytes = entries.max(1) * entry_bytes(read);
    for page in [PAGE_4K, PAGE_16K, PAGE_64K] {
        let size = page_bytes(page);
        let pages = bytes.div_ceil(size).min(MAX_PAGES);
        let region = alloc(pages * size, size).ok_or("no memory for an ITS table")?;
        regs.write64(at, table(read, region.phys, page, pages, true));
        let back = regs.read64(at);
        if page_of(back) != page {
            free(region);
            continue;
        }
        if !coherent(back) {
            regs.write64(at, table(read, region.phys, page, pages, false));
        }
        return Ok(pages * size / entry_bytes(read));
    }
    Err("the ITS took none of the table page sizes")
}
