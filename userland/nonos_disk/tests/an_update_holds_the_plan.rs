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

//! An update leaves the disk plan as it is, so it goes only where the plan
//! names a data volume inside NONOS-DATA and nothing outside it: a plan
//! with no magic, a live stick's (which names no volume), a volume past the
//! partition or too small, or an import outside it or empty is refused.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/keep_disk.rs"]
mod keep_disk;
#[path = "common/mem_disk.rs"]
mod mem_disk;

use nonos_disk::Keep::{No, Ready};
use nonos_disk::MIN_DISK_SECTORS;
use nonos_disk::{names_a_volume, plan_sector, read_installed, BlockSink, NotKept, StoreImage};
use nonos_disk_map::{MIN_VOLUME_SECTORS, PLAN_LBA};

fn plan(base: u64, sectors: u64, imports: &[(u64, u64)]) -> Vec<u8> {
    let mut s = vec![0u8; 512];
    s[..8].copy_from_slice(b"NONOSDP1");
    let words = [base, sectors, imports.len() as u64].into_iter();
    for (i, w) in words.chain(imports.iter().flat_map(|&(a, b)| [a, b])).enumerate() {
        s[8 + 8 * i..16 + 8 * i].copy_from_slice(&w.to_le_bytes());
    }
    s
}

#[test]
fn a_plan_that_reaches_outside_the_data_partition_is_refused() {
    let (mut disk, found) = keep_disk::installed(MIN_DISK_SECTORS, StoreImage::empty(), 2);
    let (d, l, mib) = (found.layout.data, found.layout, 2048);
    let refused = [
        vec![0u8; 512],
        plan(0, 0, &[(d.first + mib, 4096)]),
        plan(d.first - 1, d.sectors, &[]),
        plan(d.first, d.sectors + 1, &[]),
        plan(d.first, MIN_VOLUME_SECTORS - 1, &[]),
        plan(d.first, mib, &[(l.esp.first, 4096)]),
        plan(d.first, mib, &[(l.store.first, 512)]),
        plan(d.first, mib, &[(d.end() - 1, 1024)]),
        plan(d.first, mib, &[(d.first + mib, 0)]),
        [b"NONOSDP0".as_slice(), &plan_sector(&d)[8..]].concat(),
    ];
    for (i, s) in refused.iter().enumerate() {
        disk.write_at(PLAN_LBA, s).unwrap();
        assert_eq!(read_installed(&mut disk), No(NotKept::Plan), "plan {i}");
    }
    for s in [plan(d.first, mib, &[(d.end() - 2, 1024)]), plan_sector(&d)] {
        disk.write_at(PLAN_LBA, &s).unwrap();
        assert_eq!(read_installed(&mut disk), Ready(found));
    }
    /* What the installer asks of the disk its boot runs from. */
    for (s, names) in [(&plan_sector(&d), true), (&refused[1], false), (&refused[0], false)] {
        disk.write_at(PLAN_LBA, s).unwrap();
        assert_eq!(names_a_volume(&mut disk), names);
    }
}
