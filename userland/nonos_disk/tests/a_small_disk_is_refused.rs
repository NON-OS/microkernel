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

//! A disk that cannot hold the table and the image is refused before a
//! byte is written, and the refusal says how big it was and how big it had
//! to be. A disk large enough for the image but too small to stay FAT32
//! is refused by the geometry, also before a byte lands in the table.

mod common;

use nonos_disk::{install, WriteError};

#[test]
fn a_small_disk_is_refused() {
    let f = common::files();
    let image = common::image(&f);

    let mut tiny = common::MemDisk::new(8);
    let r = install(&mut tiny, &image, common::ENTROPY, &mut |_| {});
    assert!(matches!(r, Err(WriteError::DiskTooSmall { total_sectors: 16384, .. })), "{r:?}");
    assert!(tiny.bytes.iter().all(|&b| b == 0), "nothing was written");

    // 30 MiB holds the ~10 MiB test image plus slack, but even one-sector
    // clusters give under 65525 of them: FAT16 territory, refused.
    let mut small = common::MemDisk::new(30);
    let r = install(&mut small, &image, common::ENTROPY, &mut |_| {});
    assert!(matches!(r, Err(WriteError::Volume(_))), "{r:?}");
    assert_eq!(&small.bytes[512..520], &[0u8; 8], "no primary header was written");
}
