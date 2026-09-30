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

//! The host tools that pack a store into a test image and write a disk
//! plan keep their own copies too. A packer that disagrees with the map
//! writes a store vfs refuses as a whole; a plan tool that disagrees writes
//! a plan the kernel never reads.

#[path = "lookup/defined.rs"]
mod defined;

use defined::{magic, rhs, source, value};
use nonos_disk_map::*;

#[test]
fn the_store_packer_writes_the_container_the_map_describes() {
    let pack = source("tools/nonos-store-pack");
    let line = pack.lines().find(|l| l.starts_with("MAGIC, HDR, TOC, SEC, NAME =")).unwrap();
    let values: Vec<&str> = line.split_once('=').unwrap().1.split(',').map(str::trim).collect();
    assert_eq!(values[0], magic(&STORE_MAGIC));
    let sizes: Vec<usize> = values[1..].iter().map(|v| v.parse().unwrap()).collect();
    assert_eq!(sizes, [HEADER_LEN, ENTRY_LEN, SECTOR_SIZE, NAME_LEN]);
    assert_eq!(value(&pack, "MAX_ENTRIES"), MAX_ENTRIES as u64);
    assert_eq!(value(&pack, "MAX_PAYLOAD"), MAX_TOTAL_BYTES);
}

#[test]
fn the_plan_tool_writes_where_the_kernel_reads() {
    let image = source("tools/nonos_data_plan/image.py");
    assert_eq!(value(&image, "PLAN_LBA"), PLAN_LBA);
    assert_eq!(rhs(&image, "KEY_LBA"), "PLAN_LBA + 1");
    assert_eq!(value(&image, "KEY_LBA"), KEY_LBA);
    assert_eq!(value(&image, "RING"), HEADER_RING_SECTORS);
    let main = source("tools/nonos_data_plan/__main__.py");
    assert_eq!(value(&main, "DATA_FLOOR"), DATA_FLOOR);
}
