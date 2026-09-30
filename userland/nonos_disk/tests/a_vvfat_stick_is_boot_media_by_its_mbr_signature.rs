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

/*
 * The bug seen live: a QEMU vvfat boot stick, an MBR disk to NONOS, is
 * named by the firmware's MBR signature, partition start and length, and
 * so leaves the installer's list. One field off, no table at all, or a
 * blank disk, and it is not named.
 */

#[path = "common/fat16_sectors.rs"]
mod fat16_sectors;
#[path = "common/fat16_stick.rs"]
mod fat16_stick;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "common/named.rs"]
mod named;

use fat16_stick::{stick, SIGNATURE, SIZE, START};
use named::named;
use nonos_disk::{BootPartition, MIN_DISK_SECTORS, SIGNATURE_MBR, TABLE_MBR};

#[test]
fn a_vvfat_stick_is_boot_media_by_its_mbr_signature() {
    let mut signature = [0u8; 16];
    signature[..4].copy_from_slice(&SIGNATURE);
    let p = BootPartition {
        number: 1,
        table: TABLE_MBR,
        signature_type: SIGNATURE_MBR,
        start_lba: START,
        size_lba: SIZE,
        signature,
    };
    assert_eq!(BootPartition::parse(&p.to_bytes()), Some(p));
    assert!(named(&mut stick(true, b"loader"), p));
    assert!(!named(&mut stick(true, b"loader"), BootPartition { start_lba: START + 1, ..p }));
    assert!(!named(&mut stick(true, b"loader"), BootPartition { size_lba: SIZE - 1, ..p }));
    signature[0] ^= 1;
    assert!(!named(&mut stick(true, b"loader"), BootPartition { signature, ..p }));
    assert!(!named(&mut stick(false, b"loader"), p));
    assert!(!named(&mut mem_disk::MemDisk::new(MIN_DISK_SECTORS), p));
}
