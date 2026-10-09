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

//! Both GPT headers and the protective MBR check out the way firmware
//! checks them, the two entry arrays agree, and the four partitions lie in
//! the usable range in disk order without overlap, with the types and names
//! a firmware menu and a partition tool show. At the minimum size, and past
//! 2 TiB, where the MBR's size saturates and so does the FAT's hidden-sector
//! count.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/gpt_check.rs"]
mod gpt_check;
#[path = "common/gpt_entry.rs"]
mod gpt_entry;
#[path = "common/mem_disk.rs"]
mod mem_disk;

use gpt_check::{header, le};
use nonos_disk::{install, Region, StoreImage, MIN_DISK_SECTORS};

const TYPES: [&str; 4] = [
    "94FB361A-D500-4798-936B-06EB4685AA8F",
    "6F5AE6D3-8FE5-4836-BA18-819E9FACC900",
    "2FB2309E-8C9C-4DE4-A941-0161EB222B66",
    "C12A7328-F81F-11D2-BA4B-00A0C93EC93B",
];
const NAMES: [&str; 4] = ["NONOS-STORE", "NONOS-PLAN", "NONOS-DATA", "NONOS-ESP"];

#[test]
fn the_table_is_one_firmware_accepts() {
    let f = files::files();
    for total in [MIN_DISK_SECTORS, (1u64 << 32) + (1 << 21) + 5] {
        let mut disk = mem_disk::MemDisk::new(total);
        let (image, store) = (files::image(&f), StoreImage::empty());
        let r = install(&mut disk, &image, store, entropy::ENTROPY, &mut |_| {}).unwrap();
        let (first, end, guid, array) = header(&disk, 1);
        assert_eq!(header(&disk, total - 1), (first, end, guid.clone(), array.clone()));
        assert_eq!(guid, r.disk_guid.0);
        let entries = gpt_entry::entries(&array);
        assert_eq!(entries.len(), 4);
        for (i, (e, region)) in entries.iter().zip(Region::ALL).enumerate() {
            let x = r.layout.extent(region);
            assert_eq!((e.first, e.last), (x.first, x.last()), "{}", e.name);
            assert!(first <= e.first && e.first <= e.last && e.last <= end);
            assert!(i == 0 || entries[i - 1].last < e.first, "disk order, no overlap");
            assert_eq!((e.type_text.as_str(), e.name.as_str()), (TYPES[i], NAMES[i]));
            assert_eq!(e.attributes, if region == Region::Esp { 0 } else { 1 << 1 });
            assert_eq!(e.unique, r.partitions[i].0);
        }
        let mbr = disk.read_sectors(0, 1);
        assert!(mbr[..446].iter().chain(&mbr[462..510]).all(|&b| b == 0));
        assert_eq!(mbr[446..454], [0, 0, 2, 0, 0xEE, 0xFF, 0xFF, 0xFF]);
        assert_eq!((le(&mbr, 454, 4), le(&mbr, 458, 4)), (1, (total - 1).min(0xFFFF_FFFF)));
        assert_eq!(mbr[510..], [0x55, 0xAA]);
        let bpb = disk.read_sectors(r.layout.esp.first, 1);
        assert_eq!(le(&bpb, 28, 4), r.layout.esp.first.min(0xFFFF_FFFF), "hidden sectors");
    }
}
