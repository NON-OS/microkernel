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

//! The layout, for every disk size from the minimum up: exhaustively for
//! the first sixteen million sizes, which run through every residue of the
//! one-MiB alignment thousands of times, and around every power of two up
//! to 2^48 sectors. At each size the partitions sit in disk order, share no
//! sector, lie inside the table's usable range, keep the kernel's fixed
//! sectors, and leave nothing unused between the data volume and the ESP.
//! Below the minimum there is no layout at all.

use nonos_disk::{Extent, Layout, Region, DATA_MIN_SECTORS, ESP_SECTORS, MIN_DISK_SECTORS};
use nonos_disk_map::{DATA_FLOOR, PLAN_LBA, STORE_BASE_LBA};

const MIB: u64 = 2048;
/// Files of 25 MB, well inside the one-GiB ESP.
const FILES: u64 = 50_000;

fn check(total: u64, files: u64, esp: u64) {
    let l = Layout::plan(total, files).unwrap_or_else(|| panic!("no layout for {total}"));
    let parts = Region::ALL.map(|r| l.extent(r));
    for (i, a) in parts.iter().enumerate() {
        assert!(parts[i + 1..].iter().all(|b| a.end() <= b.first && !a.overlaps(b)), "{total}");
    }
    assert!(parts[0].first >= 34 && parts[3].last() <= total - 34, "{total}: usable range");
    assert_eq!((l.backup_array_lba, l.backup_header_lba), (total - 33, total - 1));
    let want = [(STORE_BASE_LBA, PLAN_LBA), (PLAN_LBA, DATA_FLOOR)];
    assert_eq!([l.store, l.plan], want.map(|(a, b)| Extent::new(a, b - a)));
    assert_eq!((l.data.first, l.data.end()), (DATA_FLOOR, l.esp.first), "{total}");
    assert!(l.data.sectors >= DATA_MIN_SECTORS && l.data.sectors.is_multiple_of(MIB), "{total}");
    assert_eq!((l.esp.sectors, l.esp.first % MIB, l.esp.end() % MIB), (esp, 0, 0), "{total}");
    assert!(total - l.esp.end() < 33 + MIB, "{total}: the ESP ends the disk");
}

#[test]
fn every_size_from_the_minimum_up_has_a_layout_without_overlap() {
    assert_eq!(Layout::needed(FILES), MIN_DISK_SECTORS);
    for total in MIN_DISK_SECTORS..MIN_DISK_SECTORS + (1 << 24) {
        check(total, FILES, ESP_SECTORS);
    }
    for k in 23..=48u32 {
        let (lo, hi) = ((1u64 << k) - 4096, (1u64 << k) + 4096);
        (lo.max(MIN_DISK_SECTORS)..=hi).for_each(|total| check(total, FILES, ESP_SECTORS));
    }
}

#[test]
fn an_image_past_one_gib_grows_the_esp_and_the_minimum_by_whole_mibs() {
    let files = ESP_SECTORS + 1;
    let (esp, need) = (ESP_SECTORS + MIB, MIN_DISK_SECTORS + MIB);
    assert_eq!(Layout::needed(files), need);
    assert!(Layout::plan(need - 1, files).is_none());
    (need..need + (1 << 20)).for_each(|total| check(total, files, esp));
}

#[test]
fn below_the_minimum_there_is_no_layout() {
    assert_eq!(MIN_DISK_SECTORS * 512, 2177 << 20, "the stated minimum is 2177 MiB");
    for total in (0..10_000).chain(MIN_DISK_SECTORS - (1 << 22)..MIN_DISK_SECTORS) {
        assert!(Layout::plan(total, FILES).is_none(), "{total}");
    }
}
