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

//! The tables and the tree. Every chain runs inside the volume, through no
//! free or bad cluster, to an end mark, and is the only chain through each
//! of its clusters; no allocated cluster is left outside every chain; each
//! directory starts with `.` and `..` naming itself and its parent, holds
//! no name twice in any case, and each file's chain is exactly as long as
//! its size needs. FSInfo's free count is then exact and its hint free.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use super::boot::Boot;
use super::entries::{parse, Entry, ATTR_DIR};
use crate::mem_disk::MemDisk;

pub struct Volume {
    pub boot: Boot,
    /// Every file by path from the root, `/EFI/nonos/kernel.bin`.
    pub files: BTreeMap<String, Vec<u8>>,
    pub dirs: Vec<String>,
    pub used: u64,
}

struct Walk<'a> {
    disk: &'a MemDisk,
    first: u64,
    boot: Boot,
    fat: Vec<u32>,
    owner: HashMap<u32, String>,
    files: BTreeMap<String, Vec<u8>>,
    dirs: Vec<String>,
}

const END: u32 = 0x0FFF_FFF8;
const BAD: u32 = 0x0FFF_FFF7;

pub fn check(disk: &MemDisk, first: u64) -> Volume {
    let boot = Boot::read(disk, first);
    let n = boot.fat_sectors as usize;
    let a = disk.read_sectors(first + boot.reserved, n);
    let b = disk.read_sectors(first + boot.reserved + boot.fat_sectors, n);
    assert!(a == b, "the two tables differ");
    let fat: Vec<u32> = a
        .chunks_exact(4)
        .map(|e| u32::from_le_bytes(e.try_into().unwrap()) & 0x0FFF_FFFF)
        .collect();
    assert_eq!(fat[0], 0x0FFF_FFF8, "media entry");
    assert!(fat[1] >= END, "reserved entry");
    let mut w = Walk {
        disk,
        first,
        boot,
        fat,
        owner: HashMap::new(),
        files: BTreeMap::new(),
        dirs: Vec::new(),
    };
    w.dir("", 2, 0);
    let clusters = w.boot.clusters;
    for c in 2..clusters as u32 + 2 {
        if w.fat[c as usize] != 0 {
            assert!(w.owner.contains_key(&c), "cluster {c} is allocated and in no chain");
        }
    }
    assert!(w.fat[clusters as usize + 2..].iter().all(|&e| e == 0), "entries past the volume");
    let used = w.owner.len() as u64;
    assert_eq!(w.boot.free as u64, clusters - used, "FSInfo's free count");
    let hint = w.boot.next_free;
    let free_hint = (2..clusters as u32 + 2).contains(&hint) && w.fat[hint as usize] == 0;
    assert!(hint == 0xFFFF_FFFF || free_hint, "FSInfo's next-free hint {hint}");
    Volume { boot: w.boot, files: w.files, dirs: w.dirs, used }
}

impl Walk<'_> {
    fn chain(&mut self, start: u32, who: &str) -> Vec<u32> {
        let (mut out, mut c) = (Vec::new(), start);
        loop {
            assert!((2..self.boot.clusters as u32 + 2).contains(&c), "{who}: cluster {c} outside");
            if let Some(other) = self.owner.insert(c, who.to_string()) {
                panic!("{who}: cluster {c} is also in {other}'s chain");
            }
            out.push(c);
            let next = self.fat[c as usize];
            assert!(next != 0 && next != BAD, "{who}: the chain runs into cluster {c}'s {next}");
            if next >= END {
                return out;
            }
            c = next;
        }
    }

    fn bytes(&self, chain: &[u32]) -> Vec<u8> {
        let spc = self.boot.sectors_per_cluster as usize;
        chain
            .iter()
            .flat_map(|&c| self.disk.read_sectors(self.boot.cluster_lba(self.first, c), spc))
            .collect()
    }

    fn dir(&mut self, path: &str, cluster: u32, parent: u32) {
        let chain = self.chain(cluster, if path.is_empty() { "/" } else { path });
        let entries = parse(&self.bytes(&chain));
        let rest: &[Entry] = if path.is_empty() {
            &entries
        } else {
            let dots = [(".", cluster), ("..", parent)];
            for (e, (name, at)) in entries.iter().zip(dots) {
                assert_eq!(
                    (e.name.as_str(), e.attr & ATTR_DIR, e.cluster),
                    (name, ATTR_DIR, at),
                    "{path}"
                );
            }
            &entries[2..]
        };
        let (mut shorts, mut names) = (BTreeSet::new(), BTreeSet::new());
        for e in rest {
            assert!(shorts.insert(e.short), "{path}: short name {:?} twice", e.short);
            assert!(names.insert(e.name.to_lowercase()), "{path}: {} twice in some case", e.name);
            assert!(e.name != "." && e.name != "..", "{path}: a dot entry out of place");
        }
        let cluster_bytes = self.boot.sectors_per_cluster as usize * 512;
        for e in rest {
            let child = format!("{path}/{}", e.name);
            if e.attr & ATTR_DIR != 0 {
                assert_eq!(e.size, 0, "{child}: a directory with a size");
                self.dirs.push(child.clone());
                self.dir(&child, e.cluster, if path.is_empty() { 0 } else { cluster });
                continue;
            }
            let data = if e.size == 0 {
                assert_eq!(e.cluster, 0, "{child}: an empty file with a cluster");
                Vec::new()
            } else {
                let chain = self.chain(e.cluster, &child);
                assert_eq!(
                    chain.len(),
                    (e.size as usize).div_ceil(cluster_bytes),
                    "{child}: chain"
                );
                self.bytes(&chain)[..e.size as usize].to_vec()
            };
            self.files.insert(child, data);
        }
    }
}
