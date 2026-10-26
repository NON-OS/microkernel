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

//! The loader's record of the partition it was read from names the disk to
//! the kernel by the ESP's GUID, as the installer wrote it; a record of an
//! MBR partition, of no partition, or of another length names none.

use crate::kernel_gpt::Which;
use crate::kernel_gpt::{boot_guid, entries, header, holds, Entry};
use crate::mem_disk::MemDisk;
use crate::written::written;
use nonos_disk::{BootPartition, MIN_DISK_SECTORS};
use nonos_disk::{SIGNATURE_GUID, TABLE_GPT};

fn read(disk: &MemDisk, total: u64) -> Option<Vec<Entry>> {
    let h = header::parse_as(&disk.read_sectors(1, 1), total, Which::Primary)?;
    entries(&h, &disk.read_sectors(h.entries_lba, h.array_sectors() as usize))
}

#[test]
fn the_loaders_record_of_the_esp_names_the_disk() {
    let (disk, l, guid) = written(MIN_DISK_SECTORS);
    let esp = l.esp;
    let p = BootPartition {
        number: 4,
        table: TABLE_GPT,
        signature_type: SIGNATURE_GUID,
        start_lba: esp.first,
        size_lba: esp.sectors,
        signature: guid,
    };
    assert_eq!(boot_guid(&p.to_bytes()), Some(guid));
    assert!(holds(&read(&disk, MIN_DISK_SECTORS).unwrap(), &guid));
    assert_eq!(boot_guid(&BootPartition { table: 1, signature_type: 1, ..p }.to_bytes()), None);
    assert_eq!(boot_guid(&BootPartition { signature: [0; 16], ..p }.to_bytes()), None);
    assert_eq!(boot_guid(&p.to_bytes()[..39]), None);
}
