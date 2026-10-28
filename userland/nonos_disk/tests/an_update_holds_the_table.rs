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

//! An update writes no partition table, so it goes only where the table is
//! as this writer wrote it: one with either checksum off, a header field
//! changed, a fifth entry, or an entry retyped, renamed or flagged, its
//! checksums made good again, is refused, and the table put back is not.

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
use nonos_disk::{read_installed, BlockSink, NotKept, StoreImage, MIN_DISK_SECTORS};
use table_edit::{edit, shift};

#[test]
fn a_table_this_writer_did_not_write_is_refused() {
    let (mut disk, found) = keep_disk::installed(MIN_DISK_SECTORS, StoreImage::empty(), 2);
    let head = disk.read_sectors(0, 34);
    /* Each header field one up: signature, sizes, LBAs, count, entry size. */
    let header = [0usize, 12, 24, 32, 40, 48, 72, 80, 84].map(|at| (true, at));
    /* A fifth entry's type, the store's type, the data's name, the ESP's flags. */
    let array = [4 * 128, 0, 2 * 128 + 56, 3 * 128 + 48].map(|at| (false, at));
    for (in_header, at) in header.into_iter().chain(array) {
        edit(&mut disk, |h, a| if in_header { shift(h, at, 1) } else { a[at] ^= 1 });
        assert_eq!(read_installed(&mut disk), No(NotKept::Table), "{in_header} {at}");
        disk.write_at(0, &head).unwrap();
    }
    /* The header's own checksum off; then the array's, an unused name. */
    let unused = 100 * 128 + 56;
    for (lba, at) in [(1u64, 16usize), (2 + unused as u64 / 512, unused % 512)] {
        let mut s = disk.read_sectors(lba, 1);
        s[at] ^= 1;
        disk.write_at(lba, &s).unwrap();
        assert_eq!(read_installed(&mut disk), No(NotKept::Table), "checksum, sector {lba}");
        disk.write_at(0, &head).unwrap();
    }
    disk.write_at(0, &head).unwrap();
    assert_eq!(read_installed(&mut disk), Ready(found));
}
