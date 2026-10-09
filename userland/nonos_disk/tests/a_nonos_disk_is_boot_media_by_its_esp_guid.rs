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
 * A NONOS disk booted from is named by its ESP's partition GUID, start and
 * length, as the firmware's Hard Drive node gives them; the record is
 * refused when its GUID or its partition number is off, and a stick with
 * no GPT is not named by it.
 */

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/fat16_sectors.rs"]
mod fat16_sectors;
#[path = "common/fat16_stick.rs"]
mod fat16_stick;
#[path = "common/files.rs"]
mod files;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "common/named.rs"]
mod named;

use named::named;
use nonos_disk::{install, BootPartition, StoreImage, MIN_DISK_SECTORS};
use nonos_disk::{SIGNATURE_GUID, TABLE_GPT};

fn le64(b: &[u8]) -> u64 {
    u64::from_le_bytes(b[..8].try_into().unwrap())
}

#[test]
fn a_nonos_disk_is_boot_media_by_its_esp_guid() {
    let f = files::files();
    let mut disk = mem_disk::MemDisk::new(MIN_DISK_SECTORS);
    install(&mut disk, &files::image(&f), StoreImage::empty(), entropy::ENTROPY, &mut |_| {})
        .unwrap();
    let esp = disk.read_sectors(2, 1)[384..512].to_vec();
    let (start, last) = (le64(&esp[32..]), le64(&esp[40..]));
    let signature: [u8; 16] = esp[16..32].try_into().unwrap();
    let p = BootPartition {
        number: 4,
        table: TABLE_GPT,
        signature_type: SIGNATURE_GUID,
        start_lba: start,
        size_lba: last - start + 1,
        signature,
    };
    assert_eq!(BootPartition::parse(&p.to_bytes()), Some(p));
    assert!(named(&mut disk, p));
    let mut other = signature;
    other[0] ^= 1;
    assert!(!named(&mut disk, BootPartition { signature: other, ..p }));
    assert!(!named(&mut disk, BootPartition { number: 3, ..p }));
    assert!(!named(&mut fat16_stick::stick(true, b"loader"), p));
}
