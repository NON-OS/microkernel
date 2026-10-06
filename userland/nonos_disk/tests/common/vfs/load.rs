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

//! A store image read the way vfs reads one at boot: the header's count,
//! the table decoded and bounded against the store's window, then each
//! payload at its offset, checked against its digest. The vfs leaves a
//! damaged descriptor out and loads the rest; a store the installer wrote
//! has none, so here every one must stand.

use nonos_disk_map::{digest16, SECTOR_SIZE, STORE_BASE_LBA, STORE_END_LBA, TOC_SPAN};

use super::store_header::entry_count;
use super::store_toc::{decode, Window};

/// `image` holds the disk's bytes from `STORE_BASE_LBA` on. Every file, in
/// table order, as vfs would stage it.
pub fn load(image: &[u8]) -> Vec<(String, Vec<u8>)> {
    let count = entry_count(&image[..SECTOR_SIZE]).expect("the header");
    let base = STORE_BASE_LBA * SECTOR_SIZE as u64;
    let window = Window { base, end: STORE_END_LBA * SECTOR_SIZE as u64 };
    let toc = decode(image, count, window).expect("the table");
    assert_eq!(toc.refused, 0, "a descriptor the vfs would leave out");
    toc.entries
        .into_iter()
        .enumerate()
        .map(|(slot, e)| {
            assert_eq!(e.slot, slot, "{}: read from its own slot", e.name);
            assert!(e.offset >= base + TOC_SPAN as u64, "{}: past a full table", e.name);
            let at = (e.offset - base) as usize;
            let data = image[at..at + e.len as usize].to_vec();
            assert_eq!(e.digest, digest16(&data), "{}", e.name);
            (e.name, data)
        })
        .collect()
}
