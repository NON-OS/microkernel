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

//! The reclaim on volumes the kernel's code laid out: it frees exactly what
//! rewrites and removals left unreached, a block it frees never lands on a
//! file still named, and a tree that does not add up frees nothing.

use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use super::file_consts::{INDEX_COUNT_OFFSET, INDEX_MAGIC_FLAT, INDEX_PTR_BASE};
use super::reclaim::walk::{reach, WalkError};
use super::tree_store::BlockStore;
use super::vol::{byte, Vol, LO, ROOT};
use crate::write_u64::write_u64;

/// The files a test keeps: (directory, name) to node, size, seed and blocks.
type Files = BTreeMap<(u64, Vec<u8>), (u64, u64, u64, BTreeSet<u64>)>;

fn freed(v: &Vol) -> BTreeSet<u64> {
    v.pool.iter().flat_map(|&(s, e)| s..e).collect()
}

fn check_all(v: &mut Vol, files: &Files) {
    for (node, size, seed, _) in files.values() {
        let got = v.read(*node);
        assert_eq!(got.len() as u64, *size);
        assert!(
            got.iter().enumerate().all(|(i, b)| *b == byte(*seed, i as u64)),
            "node {node} lost its bytes"
        );
    }
}

/// Writes through the volume and keeps which blocks the write took.
fn write(
    v: &mut Vol,
    files: &mut Files,
    garbage: &mut BTreeSet<u64>,
    dir: u64,
    name: &[u8],
    size: u64,
    seed: u64,
) {
    let before: BTreeSet<u64> = v.given.clone();
    let node = v.write(dir, name, size, seed);
    let mut took: BTreeSet<u64> = v.given.difference(&before).copied().collect();
    took.remove(&node);
    // A record block a first write grew the directory by is the directory's.
    took.retain(|b| !v.blocks.get(b).is_some_and(|blk| blk[..8] == *b"NONOSDR1"));
    match files.insert((dir, name.to_vec()), (node, size, seed, took)) {
        Some((_, _, _, old)) => garbage.extend(old),
        // A first write also made the node, and maybe a record block, both kept while named.
        None => {}
    }
}

fn rng(x: &mut u64) -> u64 {
    *x ^= *x << 13;
    *x ^= *x >> 7;
    *x ^= *x << 17;
    *x
}

#[test]
fn a_volume_with_nothing_rewritten_frees_nothing() {
    let mut v = Vol::new();
    let d = v.mkdir(ROOT, b"models");
    for (i, size) in [0u64, 1, 484, 485, 29_040, 1_800_000].iter().enumerate() {
        v.write(d, format!("f{i}").as_bytes(), *size, i as u64);
    }
    let r = v.reclaim();
    assert!(v.pool.is_empty(), "{:?}", v.pool);
    assert_eq!(r.runs.iter().map(|(s, e)| e - s).sum::<u64>(), v.next - LO);
}

#[test]
fn exactly_what_rewrites_and_removals_left_is_freed_and_reused_safely() {
    let (mut v, mut files, mut garbage) = (Vol::new(), Files::new(), BTreeSet::new());
    let dirs = [ROOT, v.mkdir(ROOT, b"u"), v.mkdir(ROOT, b"d")];
    let mut x = 0x2545_f491_4f6c_dd1du64;
    for round in 0..12 {
        for _ in 0..60 {
            let dir = dirs[(rng(&mut x) % 3) as usize];
            let name = format!("n{}", rng(&mut x) % 20).into_bytes();
            let size = [0, 100, 484, 3000, 40_000, 200_000][(rng(&mut x) % 6) as usize];
            if rng(&mut x) % 5 == 0 {
                if let Some((node, _, _, blocks)) = files.remove(&(dir, name.clone())) {
                    v.unlink(dir, &name);
                    garbage.extend(blocks);
                    garbage.insert(node);
                }
            } else {
                write(&mut v, &mut files, &mut garbage, dir, &name, size, rng(&mut x));
            }
            // Every step is a place a cut can fall: every named file is whole.
            check_all(&mut v, &files);
        }
        // Blocks the last reclaim freed and nothing took since are free still.
        garbage.extend(freed(&v));
        v.reclaim();
        assert_eq!(freed(&v), garbage, "round {round}: freed is not what was left");
        garbage.clear();
    }
}

#[test]
fn a_reclaim_cut_short_leaves_the_volume_as_it_was() {
    let (mut v, mut files, mut garbage) = (Vol::new(), Files::new(), BTreeSet::new());
    for i in 0..30u64 {
        write(
            &mut v,
            &mut files,
            &mut garbage,
            ROOT,
            format!("a{}", i % 7).as_bytes(),
            5000 + i * 300,
            i,
        );
    }
    let blocks = v.blocks.clone();
    v.reclaim();
    // A reclaim writes nothing: a cut during it, or before the first reuse,
    // finds every block as the walk found it.
    assert_eq!(v.blocks, blocks);
    v.pool.clear();
    check_all(&mut v, &files);
    v.reclaim();
    assert_eq!(freed(&v), garbage);
}

#[test]
fn a_file_of_the_flat_layout_is_reached() {
    let mut v = Vol::new();
    let data: Vec<u64> = (0..5).map(|_| v.alloc().unwrap()).collect();
    let stray = v.alloc().unwrap();
    let index = v.alloc().unwrap();
    let mut b = [0u8; 484];
    b[..8].copy_from_slice(&INDEX_MAGIC_FLAT);
    b[INDEX_COUNT_OFFSET..INDEX_COUNT_OFFSET + 4].copy_from_slice(&5u32.to_le_bytes());
    for (i, d) in data.iter().enumerate() {
        write_u64(&mut b, INDEX_PTR_BASE + i * 8, *d);
        v.blocks.insert(*d, [1u8; 484]);
    }
    v.blocks.insert(index, b);
    let node = v.alloc().unwrap();
    v.nodes.insert(node, (false, index, 5 * 484));
    v.unlink(ROOT, b"none");
    let mut root_dir = Vol::new();
    std::mem::swap(&mut root_dir.blocks, &mut v.blocks);
    root_dir.nodes = v.nodes.clone();
    root_dir.next = v.next;
    let mut v = root_dir;
    let rec_owner = v.mkdir(ROOT, b"keep");
    v.nodes.insert(rec_owner, (false, index, 5 * 484));
    v.reclaim();
    let free = freed(&v);
    assert!(free.contains(&stray) && free.contains(&node));
    assert!(data.iter().all(|d| !free.contains(d)) && !free.contains(&index));
}

#[test]
fn a_tree_that_does_not_add_up_frees_nothing() {
    let mut v = Vol::new();
    let node = v.write(ROOT, b"f", 100_000, 1);
    let index = v.nodes[&node].1;
    let mut b = v.blocks[&index];
    write_u64(&mut b, INDEX_PTR_BASE, v.next + 10);
    v.blocks.insert(index, b);
    let next = v.next;
    assert_eq!(reach(&mut v, ROOT, LO, next).err(), Some(WalkError::Shape));
    let mut b = v.blocks[&index];
    b[..8].copy_from_slice(b"NOTINDEX");
    v.blocks.insert(index, b);
    assert_eq!(reach(&mut v, ROOT, LO, next).err(), Some(WalkError::Shape));
    v.blocks.remove(&index);
    assert_eq!(reach(&mut v, ROOT, LO, next).err(), Some(WalkError::Read("no block")));
}

/// Host numbers for the notes: run with `-- --ignored --nocapture`.
#[test]
#[ignore]
fn how_long_a_reclaim_takes_on_a_large_volume() {
    let mut v = Vol::new();
    let d = v.mkdir(ROOT, b"m");
    for i in 0..4u64 {
        v.write(d, format!("model{i}").as_bytes(), 256 << 20, i);
    }
    for i in 0..2000u64 {
        v.write(d, format!("doc{i}").as_bytes(), 40_000, i);
    }
    let t = Instant::now();
    let r = v.reclaim();
    let took = t.elapsed();
    let blocks = v.next - LO;
    println!(
        "reclaim: {} blocks ({} MiB) on the volume, {} read, {} nodes, {:?} on the host's memory",
        blocks,
        blocks * 484 >> 20,
        r.reads,
        r.nodes,
        took
    );
}
