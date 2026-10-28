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

//! What an update makes of an installed disk's key header: kept for a
//! passphrase or a wrapped key handed over; waiting for a wrapped key not
//! handed over; not kept for a bound key, handover or not, no header, or an
//! unknown way. A flipped bit in either magic is that magic absent.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/keep_disk.rs"]
mod keep_disk;
#[path = "common/mem_disk.rs"]
mod mem_disk;

use keep_disk::{installed, key_header};
use nonos_disk::Keep::{NeedsPrepare, No, Ready};
use nonos_disk::{read_installed, BlockSink, NotKept, StoreImage, MIN_DISK_SECTORS};
use nonos_disk_map::KEY_LBA;

#[test]
fn each_way_gives_its_answer() {
    let (mut disk, found) = installed(MIN_DISK_SECTORS, StoreImage::empty(), 2);
    let mut cases = vec![
        (key_header(2, false), Ready(found)),
        (key_header(2, true), Ready(found)),
        (key_header(3, true), Ready(found)),
        (key_header(3, false), NeedsPrepare(found)),
        (key_header(1, false), No(NotKept::MadeBefore)),
        (key_header(1, true), No(NotKept::MadeBefore)),
        ([0u8; 512], No(NotKept::NoKeyHeader)),
    ];
    for way in [0u8, 4, 5, 128, 255] {
        cases.push((key_header(way, way % 2 == 1), No(NotKept::UnknownWay(way))));
    }
    for (header, want) in cases {
        disk.write_at(KEY_LBA, &header).unwrap();
        let case = (header[8], header[128]);
        assert_eq!(read_installed(&mut disk), want, "way and handover {case:?}");
    }
}

#[test]
fn a_flipped_bit_in_either_magic_is_that_magic_absent() {
    let (mut disk, found) = installed(MIN_DISK_SECTORS, StoreImage::empty(), 2);
    for at in 0..8 {
        for bit in 0..8 {
            for way in [2u8, 3] {
                let mut header = key_header(way, true);
                header[at] ^= 1 << bit;
                disk.write_at(KEY_LBA, &header).unwrap();
                assert_eq!(read_installed(&mut disk), No(NotKept::NoKeyHeader), "{at}.{bit}");
            }
            let mut header = key_header(3, true);
            header[128 + at] ^= 1 << bit;
            disk.write_at(KEY_LBA, &header).unwrap();
            assert_eq!(read_installed(&mut disk), NeedsPrepare(found), "handover {at}.{bit}");
        }
    }
}
