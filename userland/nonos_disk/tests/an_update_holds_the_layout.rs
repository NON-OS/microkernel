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

//! An update goes only where the table is this writer's layout: each of
//! the four partitions' first or last sector a sector off, with both
//! checksums made good so the extent is all that is wrong, is refused on
//! the smallest disk and on one past 2 TiB, and so is a boot partition of
//! part MiBs, under one GiB, or ending short of where the writer puts it.

#[path = "common/gpt_sweep/crc.rs"]
mod crc;
#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/keep_disk.rs"]
mod keep_disk;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "common/table_edit.rs"]
mod table_edit;

use nonos_disk::Keep::{No, Ready};
use nonos_disk::{read_installed, NotKept, StoreImage, MIN_DISK_SECTORS};
use table_edit::{edit, shift};

#[test]
fn every_extent_a_sector_off_is_refused() {
    for total in [MIN_DISK_SECTORS, (1u64 << 32) + (1 << 21) + 77] {
        let (mut disk, found) = keep_disk::installed(total, StoreImage::empty(), 2);
        assert_eq!(read_installed(&mut disk), Ready(found), "{total}");
        for i in 0..4 {
            for by in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
                let move_by = |a: &mut [u8], k: i64| {
                    shift(a, i * 128 + 32, k * by[0]);
                    shift(a, i * 128 + 40, k * by[1]);
                };
                edit(&mut disk, |_, a| move_by(a, 1));
                assert_eq!(read_installed(&mut disk), No(NotKept::Moved), "{total}: {i} {by:?}");
                edit(&mut disk, |_, a| move_by(a, -1));
            }
        }
        /*
         * The boundary a sector back, so the boot partition is part MiBs; a
         * MiB on, so it is under one GiB; both a MiB short of the end. Each
         * is refused by its own rule alone.
         */
        for (by, last) in [(-1, 0), (2048, 0), (-2048, -2048)] {
            let shift_by = |a: &mut [u8], k: i64| {
                shift(a, 2 * 128 + 40, k * by);
                shift(a, 3 * 128 + 32, k * by);
                shift(a, 3 * 128 + 40, k * last);
            };
            edit(&mut disk, |_, a| shift_by(a, 1));
            assert_eq!(read_installed(&mut disk), No(NotKept::Moved), "{total}: {by} {last}");
            edit(&mut disk, |_, a| shift_by(a, -1));
        }
        assert_eq!(read_installed(&mut disk), Ready(found), "{total}: put back");
    }
}
