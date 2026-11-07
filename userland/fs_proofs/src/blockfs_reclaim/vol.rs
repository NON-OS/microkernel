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

//! A volume in memory, laid out by the kernel's own tree writer, index
//! encoder and directory-record layout, with an allocator that hands out the
//! reclaim's free runs before the bump pointer, as alloc_block does.

#![cfg(test)]

use std::collections::{BTreeMap, BTreeSet};

use super::dir_block_header::{count, has_room, is_record, next, set_count, set_next};
use super::dir_consts::{ENTRY_BYTES, NAME_BYTES, REC_ENTRY_BASE, REC_MAGIC};
use super::file_consts::{DATA_BYTES, ROOTS};
use super::index_block::encode;
use super::reclaim::runs::gaps;
use super::reclaim::walk::{reach, Blocks, Reached};
use super::tree_reader::TreeReader;
use super::tree_store::{Block, BlockSource, BlockStore};
use super::tree_writer::TreeWriter;
use crate::read_u64::read_u64;
use crate::write_u64::write_u64;

/// The header ring's end: the first block a volume gives out, the root's.
pub const LO: u64 = 256;
pub const ROOT: u64 = LO;

#[derive(Default)]
pub struct Vol {
    pub blocks: BTreeMap<u64, Block>,
    /// Each node: whether a directory, its first record or index block, its size.
    pub nodes: BTreeMap<u64, (bool, u64, u64)>,
    pub next: u64,
    pub pool: Vec<(u64, u64)>,
    /// Every block given out since the last reclaim.
    pub given: BTreeSet<u64>,
}

impl BlockSource for Vol {
    type Error = &'static str;
    fn get(&mut self, lba: u64) -> Result<Block, &'static str> {
        self.blocks.get(&lba).copied().ok_or("no block")
    }
}

impl BlockStore for Vol {
    fn alloc(&mut self) -> Result<u64, &'static str> {
        let lba = loop {
            match self.pool.last_mut() {
                Some(run) if run.0 < run.1 => {
                    run.0 += 1;
                    break run.0 - 1;
                }
                Some(_) => {
                    self.pool.pop();
                }
                None => {
                    self.next += 1;
                    break self.next - 1;
                }
            }
        };
        assert!(self.given.insert(lba), "block {lba} given out twice since the reclaim");
        Ok(lba)
    }
    fn put(&mut self, lba: u64, block: &Block) -> Result<(), &'static str> {
        self.blocks.insert(lba, *block);
        Ok(())
    }
}

impl Blocks for Vol {
    type Error = &'static str;
    fn node(&mut self, lba: u64) -> Result<(bool, u64), &'static str> {
        self.nodes.get(&lba).map(|n| (n.0, n.1)).ok_or("no node")
    }
    fn block(&mut self, lba: u64) -> Result<Block, &'static str> {
        self.get(lba)
    }
}

/// The byte file `seed` holds at `i`.
pub fn byte(seed: u64, i: u64) -> u8 {
    (i.wrapping_add(seed.wrapping_mul(7919)).wrapping_mul(2_654_435_761) >> 13) as u8
}

impl Vol {
    pub fn new() -> Vol {
        let mut v = Vol { next: LO + 1, ..Vol::default() };
        v.nodes.insert(ROOT, (true, 0, 0));
        v
    }

    /// Every entry of directory `dir`: name and node.
    pub fn entries(&mut self, dir: u64) -> Vec<(Vec<u8>, u64)> {
        let mut out = Vec::new();
        let mut rec = self.nodes[&dir].1;
        while rec != 0 {
            let b = self.blocks[&rec];
            assert!(is_record(&b));
            for i in 0..count(&b) {
                let at = REC_ENTRY_BASE + i * ENTRY_BYTES;
                let name = &b[at..at + NAME_BYTES];
                let end = name.iter().position(|c| *c == 0).unwrap_or(NAME_BYTES);
                if end > 0 {
                    out.push((name[..end].to_vec(), read_u64(&b, at + NAME_BYTES)));
                }
            }
            rec = next(&b);
        }
        out
    }

    fn link(&mut self, dir: u64, name: &[u8], node: u64) {
        let mut rec = self.nodes[&dir].1;
        let mut last = 0;
        while rec != 0 && !has_room(&self.blocks[&rec]) {
            last = rec;
            rec = next(&self.blocks[&rec]);
        }
        if rec == 0 {
            rec = self.alloc().unwrap();
            let mut b = [0u8; DATA_BYTES];
            b[..8].copy_from_slice(&REC_MAGIC);
            self.blocks.insert(rec, b);
            match last {
                0 => self.nodes.get_mut(&dir).unwrap().1 = rec,
                l => set_next(self.blocks.get_mut(&l).unwrap(), rec),
            }
        }
        let b = self.blocks.get_mut(&rec).unwrap();
        let i = count(b);
        let at = REC_ENTRY_BASE + i * ENTRY_BYTES;
        b[at..at + name.len()].copy_from_slice(name);
        write_u64(b, at + NAME_BYTES, node);
        set_count(b, i + 1);
    }

    /// Takes `name` out of `dir`, its name padded away as a removed entry is.
    pub fn unlink(&mut self, dir: u64, name: &[u8]) {
        let mut rec = self.nodes[&dir].1;
        while rec != 0 {
            let b = self.blocks.get_mut(&rec).unwrap();
            for i in 0..count(b) {
                let at = REC_ENTRY_BASE + i * ENTRY_BYTES;
                if b[at..at + name.len()] == *name && b[at + name.len()] == 0 {
                    b[at..at + NAME_BYTES].fill(0);
                    return;
                }
            }
            rec = next(b);
        }
    }

    pub fn mkdir(&mut self, dir: u64, name: &[u8]) -> u64 {
        let node = self.alloc().unwrap();
        self.nodes.insert(node, (true, 0, 0));
        self.link(dir, name, node);
        node
    }

    /// Writes `size` bytes of `seed` as `name` in `dir`, over what it held:
    /// new blocks, as write_file gives them, and the old ones unreached.
    pub fn write(&mut self, dir: u64, name: &[u8], size: u64, seed: u64) -> u64 {
        let mut tree = TreeWriter::new();
        let mut at = 0u64;
        while at < size {
            let mut block = [0u8; DATA_BYTES];
            let take = (size - at).min(DATA_BYTES as u64) as usize;
            for (k, b) in block[..take].iter_mut().enumerate() {
                *b = byte(seed, at + k as u64);
            }
            let lba = self.alloc().unwrap();
            self.put(lba, &block).unwrap();
            tree.push(self, lba).unwrap();
            at += take as u64;
        }
        tree.finish(self).unwrap();
        let index = self.alloc().unwrap();
        let block = encode(&tree.root, tree.data_blocks);
        self.put(index, &block).unwrap();
        match self.entries(dir).into_iter().find(|(n, _)| n == name) {
            Some((_, node)) => {
                self.nodes.insert(node, (false, index, size));
                node
            }
            None => {
                let node = self.alloc().unwrap();
                self.nodes.insert(node, (false, index, size));
                self.link(dir, name, node);
                node
            }
        }
    }

    /// The bytes of the file at `node`, read through the kernel's tree reader.
    pub fn read(&mut self, node: u64) -> Vec<u8> {
        let (_, index, size) = self.nodes[&node];
        let block = self.blocks[&index];
        let roots: [u64; ROOTS] = match super::index_block::decode(&block) {
            Some(super::index_block::Index::Tree(r)) => r,
            _ => panic!("not a tree index"),
        };
        let mut reader = TreeReader::new(roots);
        let mut out = Vec::new();
        let mut n = 0u64;
        while (out.len() as u64) < size {
            let lba = reader.data_lba(self, n).unwrap();
            let b = self.blocks[&lba];
            let take = (size - out.len() as u64).min(DATA_BYTES as u64) as usize;
            out.extend_from_slice(&b[..take]);
            n += 1;
        }
        out
    }

    /// The reclaim, as the volume layer runs it: walk, then free the gaps.
    pub fn reclaim(&mut self) -> Reached {
        let next = self.next;
        let reached = reach(self, ROOT, LO, next).unwrap();
        self.pool = gaps(LO, next, &reached.runs);
        self.given.clear();
        reached
    }
}
