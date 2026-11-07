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

//! Every block the tree reaches from its root: each node, each directory's
//! record blocks, each file's index, pointer and data blocks. Generic over
//! where blocks come from, so host proofs walk volumes this layout wrote.

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use super::super::dir_block::{lba, name};
use super::super::dir_block_header::{count, is_record, next};
use super::super::dir_consts::MAX_RECORDS;
use super::super::file_consts::{
    DIRECT_SLOTS, FANOUT, INDEX_COUNT_OFFSET, INDEX_PTR_BASE, LEVELS, MAX_PTRS, PTR_BYTES,
};
use super::super::index_block::{decode, Index};
use super::super::read_u32::read_u32;
use super::super::read_u64::read_u64;
use super::super::tree_ptrs::entry;
use super::super::tree_store::Block;
use super::runs::Runs;

/// Where a walk reads from.
pub(crate) trait Blocks {
    type Error;
    /// A node: whether it is a directory, and its first record or index block.
    fn node(&mut self, lba: u64) -> Result<(bool, u64), Self::Error>;
    /// Any other block, opened.
    fn block(&mut self, lba: u64) -> Result<Block, Self::Error>;
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum WalkError<E> {
    /// The device or the seal refused a block.
    Read(E),
    /// A block that is not what its pointer says, or a pointer outside the
    /// blocks given out: nothing is freed on a tree that does not add up.
    Shape,
}

/// What a walk reached, and what it read to learn it.
pub(crate) struct Reached {
    pub runs: Vec<(u64, u64)>,
    pub nodes: u64,
    pub reads: u64,
}

/// Every block reached from the node at `root`, each in [`lo`, `hi`).
pub(crate) fn reach<B: Blocks>(
    b: &mut B,
    root: u64,
    lo: u64,
    hi: u64,
) -> Result<Reached, WalkError<B::Error>> {
    let mut w = Walk { b, lo, hi, runs: Runs::default(), reads: 0 };
    let mut nodes = BTreeSet::new();
    let mut todo = alloc::vec![root];
    while let Some(at) = todo.pop() {
        // A node reached twice (a second link) is walked once.
        if !nodes.insert(at) {
            continue;
        }
        w.mark(at)?;
        let (dir, first) = w.b.node(at).map_err(WalkError::Read)?;
        w.reads += 1;
        match (dir, first) {
            (_, 0) => {}
            (true, first) => w.dir(first, &mut todo)?,
            (false, first) => w.file(first)?,
        }
    }
    Ok(Reached { runs: w.runs.settle(), nodes: nodes.len() as u64, reads: w.reads })
}

struct Walk<'a, B: Blocks> {
    b: &'a mut B,
    lo: u64,
    hi: u64,
    runs: Runs,
    reads: u64,
}

impl<B: Blocks> Walk<'_, B> {
    fn mark(&mut self, lba: u64) -> Result<(), WalkError<B::Error>> {
        if lba < self.lo || lba >= self.hi {
            return Err(WalkError::Shape);
        }
        self.runs.add(lba);
        Ok(())
    }

    fn read(&mut self, lba: u64) -> Result<Block, WalkError<B::Error>> {
        self.mark(lba)?;
        self.reads += 1;
        self.b.block(lba).map_err(WalkError::Read)
    }

    fn dir(&mut self, mut rec: u64, todo: &mut Vec<u64>) -> Result<(), WalkError<B::Error>> {
        for _ in 0..MAX_RECORDS {
            if rec == 0 {
                return Ok(());
            }
            let block = self.read(rec)?;
            if !is_record(&block) {
                return Err(WalkError::Shape);
            }
            for i in 0..count(&block) {
                // An all-padding name is a removed entry (dir_list.rs).
                if name(&block, i).first().is_some_and(|c| *c != 0) {
                    todo.push(lba(&block, i));
                }
            }
            rec = next(&block);
        }
        Err(WalkError::Shape)
    }

    fn file(&mut self, index: u64) -> Result<(), WalkError<B::Error>> {
        let block = self.read(index)?;
        match decode(&block).ok_or(WalkError::Shape)? {
            Index::Flat(flat) => {
                let n = (read_u32(&flat, INDEX_COUNT_OFFSET) as usize).min(MAX_PTRS);
                for i in 0..n {
                    let at = read_u64(&flat, INDEX_PTR_BASE + i * PTR_BYTES);
                    if at != 0 {
                        self.mark(at)?;
                    }
                }
            }
            Index::Tree(roots) => {
                for &at in roots[..DIRECT_SLOTS].iter().filter(|a| **a != 0) {
                    self.mark(at)?;
                }
                for depth in 1..=LEVELS {
                    let top = roots[DIRECT_SLOTS + depth - 1];
                    if top != 0 {
                        self.tree(top, depth)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// A pointer tree `depth` levels above its data.
    fn tree(&mut self, top: u64, depth: usize) -> Result<(), WalkError<B::Error>> {
        let mut todo = alloc::vec![(top, depth)];
        while let Some((at, depth)) = todo.pop() {
            let block = self.read(at)?;
            for i in 0..FANOUT {
                match (entry(&block, i), depth) {
                    (0, _) => {}
                    (data, 1) => self.mark(data)?,
                    (ptr, d) => todo.push((ptr, d - 1)),
                }
            }
        }
        Ok(())
    }
}
