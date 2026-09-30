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

//! A store for files of tens of gigabytes. Data block n is only numbered,
//! DATA + n, and reads as the bytes made from that number. A pointer block
//! that is an even run of LBAs, then zeros, is kept as its first, step and
//! count, and any other whole, so 1.4 million of them fit in a few MB.

use std::collections::HashMap;

use super::file_consts::FANOUT;
use super::mem::numbered;
use super::tree_ptrs::{encode, entry};
use super::tree_store::{Block, BlockSource, BlockStore};

/// The LBA of data block 0.
pub const DATA: u64 = 1 << 40;

type Run = (u64, u64, u8);

#[derive(Default)]
pub struct Packed {
    runs: Vec<Run>,
    whole: HashMap<u64, Block>,
}

fn unpack(&(first, step, count): &Run) -> Block {
    let mut ptrs = [0u64; FANOUT];
    for (i, p) in ptrs[..count as usize].iter_mut().enumerate() {
        *p = first.wrapping_add(step.wrapping_mul(i as u64));
    }
    encode(&ptrs)
}

fn pack(b: &Block) -> Run {
    let count = (0..FANOUT).take_while(|&i| entry(b, i) != 0).count() as u8;
    (entry(b, 0), entry(b, 1).wrapping_sub(entry(b, 0)), count)
}

impl BlockSource for Packed {
    type Error = &'static str;
    fn get(&mut self, lba: u64) -> Result<Block, &'static str> {
        if lba >= DATA {
            return Ok(numbered(lba));
        }
        if let Some(b) = self.whole.get(&lba) {
            return Ok(*b);
        }
        let i = lba.checked_sub(1).ok_or("lba 0")? as usize;
        self.runs.get(i).map(unpack).ok_or("not a pointer block")
    }
}

impl BlockStore for Packed {
    fn alloc(&mut self) -> Result<u64, &'static str> {
        self.runs.push((0, 0, 0));
        Ok(self.runs.len() as u64)
    }
    fn put(&mut self, lba: u64, b: &Block) -> Result<(), &'static str> {
        let i = lba.checked_sub(1).ok_or("lba 0")? as usize;
        let run = pack(b);
        *self.runs.get_mut(i).ok_or("not allocated")? = run;
        self.whole.remove(&lba);
        if unpack(&run) != *b {
            self.whole.insert(lba, *b);
        }
        Ok(())
    }
}
