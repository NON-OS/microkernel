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

//! The writer's directory encoding and volume geometry, from its own source
//! (`src/fat32/`), at the edges the shipped tree never reaches: long names
//! whose aliases collide with each other and with real short names, names
//! that differ only in case, a volume with no free cluster, and partition
//! sizes at every cluster-size boundary from the smallest FAT32 volume to
//! the largest. Directories are read back by `common/fat_check`.

extern crate alloc;

/*
 * Part of the crate's source, mounted for the parts under test: what the
 * rest of the crate uses of it goes unused here.
 */
#[path = "common/fat_check/entries.rs"]
#[allow(dead_code)]
mod entries;
#[path = "common/writer_fat32/mod.rs"]
#[allow(dead_code, unused_imports)]
mod fat32;
#[path = "../src/sink.rs"]
#[allow(dead_code)]
mod sink;

use std::collections::BTreeSet;

use fat32::bpb::fs_info;
use fat32::dir::encode;
use fat32::geometry::{plan, PlanError, FAT_COUNT, RESERVED_SECTORS};
use fat32::tree::{place_with, Node};
use fat32::write::check::{check, fits_a_slot};

/// The root directory holding one empty file per name, as written.
fn root(names: &[&str]) -> Result<Vec<u8>, fat32::dir::NameError> {
    let tree: Vec<Node<'_>> = names.iter().map(|n| Node::file(n, &[])).collect();
    let placed = place_with(&tree, 4096);
    let mut bytes = encode(0, &placed.runs)?;
    bytes.resize(placed.runs[0].clusters as usize * 4096, 0);
    Ok(bytes)
}

fn read_back(names: &[&str]) {
    let entries = entries::parse(&root(names).expect("encodes"));
    let got: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(got, names);
    let shorts: BTreeSet<[u8; 11]> = entries.iter().map(|e| e.short).collect();
    assert_eq!(shorts.len(), names.len(), "two slots share a short name");
}

#[test]
fn aliases_never_collide_with_each_other_or_with_a_short_name() {
    read_back(&[
        "bootloader.trailer",
        "BOOTLO~1.TRA",
        "bootlo~2.tra",
        "bootloader.trailers",
        "Bootloader.TRA",
        "boot loader.trailer",
        "bootloader.a.b.c",
        ".hidden",
        "a+b.c",
        "caf\u{e9}.txt",
        &"n".repeat(255),
    ]);
}

#[test]
fn a_thousand_long_names_alike_get_a_thousand_aliases() {
    let names: Vec<String> = (0..1000).map(|i| format!("bootloader.trailer{i}")).collect();
    read_back(&names.iter().map(String::as_str).collect::<Vec<_>>());
}

/*
 * FAT matches names without regard to case. Two children that differ only
 * in case are one name twice: as short names they are two slots with the
 * same eleven bytes, and either way a driver finds whichever comes first.
 */
#[test]
fn names_that_differ_only_in_case_are_refused() {
    let pairs = [
        ["kernel.bin", "KERNEL.BIN"],
        ["Kernel.bin", "kernel.bin"],
        ["boot_root.approval", "BOOT_ROOT.APPROVAL"],
        ["Bootloader.Trailer", "bootloader.trailer"],
    ];
    for pair in pairs {
        assert!(root(&pair).is_err(), "{pair:?} written as two names");
    }
    assert!(root(&["kernel.bin", "kernel.bin2"]).is_ok());
}

#[test]
fn a_volume_with_no_free_cluster_names_no_next_free_cluster() {
    let geo = plan(1 << 21).unwrap();
    for used in [0, 1, geo.data_clusters / 2, geo.data_clusters - 1, geo.data_clusters] {
        let s = fs_info(&geo, used);
        let free = u32::from_le_bytes(s[488..492].try_into().unwrap()) as u64;
        let next = u32::from_le_bytes(s[492..496].try_into().unwrap());
        assert_eq!(free, geo.data_clusters - used);
        let in_volume = (2..geo.data_clusters + 2).contains(&(next as u64));
        assert!(next == u32::MAX || in_volume, "{used} used: next free {next}");
    }
}

/// A geometry is what a driver would derive back from its boot sector,
/// FAT32 by count, with a table that holds every cluster.
fn sound(partition: u64) -> Result<u64, PlanError> {
    let g = plan(partition)?;
    let spc = g.sectors_per_cluster as u64;
    let data = partition - RESERVED_SECTORS as u64 - FAT_COUNT as u64 * g.fat_sectors as u64;
    assert_eq!(data / spc, g.data_clusters, "{partition}: a driver counts another number");
    assert!((65_525..=0x0FFF_FFF5).contains(&g.data_clusters), "{partition}: not FAT32");
    assert!((g.data_clusters + 2) * 4 <= g.fat_sectors as u64 * 512, "{partition}: table");
    Ok(spc)
}

#[test]
fn every_partition_size_is_fat32_by_count_or_refused() {
    let smallest = (0..80_000u64).map(|s| 65_525 + s).find(|&p| plan(p).is_ok()).unwrap();
    assert_eq!(plan(smallest - 1), Err(PlanError::TooSmall));
    let mut boundaries = vec![smallest];
    let mut last = sound(smallest).unwrap();
    for p in smallest..smallest + 4_000_000 {
        let spc = sound(p).unwrap();
        if spc != last {
            boundaries.push(p);
            last = spc;
        }
    }
    assert_eq!(boundaries.len(), 4, "1, 2, 4 and 8 sectors per cluster: {boundaries:?}");
    let esp = 1u64 << 21;
    (0..4096).for_each(|mib| assert_eq!(sound(esp + mib * 2048), Ok(8)));
    let shipped = plan(esp).unwrap();
    assert_eq!((shipped.fat_sectors, shipped.data_clusters), (2044, 261_629), "the one-GiB ESP");
    /*
     * The largest volume, halving between one that fits and one that does
     * not: four-KiB clusters to the last count FAT32 allows, plus tables.
     */
    let (mut fits, mut over) = (0x0FFF_FFF5u64 * 8, 0x0FFF_FFF5u64 * 9);
    assert!(plan(fits).is_ok() && plan(over) == Err(PlanError::TooLarge));
    while over - fits > 1 {
        let mid = fits + (over - fits) / 2;
        if plan(mid).is_ok() {
            fits = mid;
        } else {
            over = mid;
        }
    }
    let largest = fits;
    assert_eq!(plan(largest).map(|g| g.data_clusters), Ok(0x0FFF_FFF5));
    (largest - 100_000..=largest).for_each(|p| assert_eq!(sound(p), Ok(8)));
    assert_eq!(plan(largest + 1), Err(PlanError::TooLarge));
    assert_eq!(plan(u64::MAX / 2), Err(PlanError::TooLarge));
}

/*
 * The plan checks the whole tree before the first sector goes out. A name
 * no slot can spell, or spells twice, used to reach the queue, which took
 * the failed encoding as an empty directory and wrote a volume without the
 * files under it.
 */
#[test]
fn a_tree_that_cannot_be_written_is_refused_before_any_write() {
    let leaf = |name| Node::file(name, &[]);
    let bad = [
        vec![Node::dir("EFI", vec![leaf("kernel.bin"), leaf("KERNEL.BIN")])],
        vec![Node::dir("EFI", vec![Node::dir("a", vec![leaf("x"), leaf("X")])])],
        vec![Node::dir("EFI", vec![leaf("a:b")])],
        vec![leaf("")],
    ];
    for tree in &bad {
        assert!(check(&place_with(tree, 4096)).is_err());
    }
    let boot = Node::dir("BOOT", vec![leaf("BOOTX64.EFI")]);
    let good = vec![Node::dir("EFI", vec![boot, leaf("kernel.bin")])];
    assert_eq!(check(&place_with(&good, 4096)), Ok(()));
    assert!(fits_a_slot(u32::MAX as usize) && !fits_a_slot(u32::MAX as usize + 1));
}
