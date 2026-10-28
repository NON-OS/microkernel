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

//! The installer's reading of the key header (`keep/key.rs`) against the
//! kernel's own handover record (`handover/place.rs`): an update is ready
//! on a passphrase, or on way 3 with a record the kernel finds; needs a
//! prepare on way 3 with none; and is offered on no other way.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/keep_disk.rs"]
mod keep_disk;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "../../../src/fs/blockfs_volume/handover/place.rs"]
mod place;

use nonos_disk::{read_installed, BlockSink, Keep, StoreImage, MIN_DISK_SECTORS};
use nonos_disk_map::KEY_LBA;

/// `s` with a handover record put where the kernel puts one, or cleared.
fn with_record(s: &[u8; 512], present: bool) -> [u8; 512] {
    let r = place::Record { nonce: [5; 12], key_and_tag: [7; 48] };
    place::put(s, present.then_some(&r))
}

#[test]
fn the_installer_offers_what_the_kernel_will_open() {
    let (mut disk, _) = keep_disk::installed(MIN_DISK_SECTORS, StoreImage::empty(), 3);
    for way in [0u8, 1, 2, 3, 4, 255] {
        for record in [false, true] {
            let s = with_record(&keep_disk::key_header(way, false), record);
            disk.write_at(KEY_LBA, &s).unwrap();
            let opens = match way {
                2 => Some(true),
                3 => Some(place::read(&s).is_some()),
                _ => None,
            };
            let offered = match read_installed(&mut disk) {
                Keep::Ready(_) => Some(true),
                Keep::NeedsPrepare(_) => Some(false),
                Keep::No(_) => None,
            };
            assert_eq!(offered, opens, "way {way}, record {record}");
        }
    }
}

#[test]
fn the_installers_handover_bytes_are_the_kernels() {
    assert!(place::read(&keep_disk::key_header(3, true)).is_some());
    let mut torn = with_record(&keep_disk::key_header(3, false), true);
    torn[135] ^= 1;
    assert!(place::read(&torn).is_none(), "one magic byte off is no record");
    let (mut disk, _) = keep_disk::installed(MIN_DISK_SECTORS, StoreImage::empty(), 3);
    disk.write_at(KEY_LBA, &torn).unwrap();
    assert!(matches!(read_installed(&mut disk), Keep::NeedsPrepare(_)));
}
