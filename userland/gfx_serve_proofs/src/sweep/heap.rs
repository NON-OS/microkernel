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

//! Memory for a driver's primaries on the host, for the wire sweep: pages
//! up to a limit, each region its own zeroed buffer, every release checked.
//! Shared by the driver proofs that mount the sweep (`server_sweep.rs`).

use std::collections::HashMap;

use super::kit::{Errno, Platform, Region, ENOMEM};

pub struct Heap {
    pub pages_left: usize,
    pub next: usize,
    pub memory: HashMap<usize, Vec<u8>>,
    pub handles: u64,
}

impl Heap {
    pub fn new(pages: usize) -> Self {
        Heap { pages_left: pages, next: 0x1000_0000, memory: HashMap::new(), handles: 1 }
    }
}

impl Platform for Heap {
    fn alloc(&mut self, len: usize) -> Result<Region, Errno> {
        let pages = len.div_ceil(4096).max(1);
        if pages > self.pages_left {
            return Err(ENOMEM);
        }
        let base = self.next;
        (self.pages_left, self.next) = (self.pages_left - pages, base + pages * 4096);
        self.memory.insert(base, vec![0; pages * 4096]);
        Ok(Region { base, len: pages * 4096 })
    }

    fn register(&mut self, _: Region, _: u32, _: u32, _: u32) -> Result<u64, Errno> {
        self.handles += 1;
        Ok(self.handles)
    }

    fn share(&mut self, _: u64, _: u32) -> Result<(), Errno> {
        Ok(())
    }

    fn release(&mut self, _: Option<u64>, r: Region) {
        let m = self.memory.remove(&r.base).expect("a region freed twice or never made");
        self.pages_left += m.len() / 4096;
    }

    fn bytes(&self, r: Region) -> &[u8] {
        &self.memory[&r.base]
    }
    fn scrub(&mut self, r: Region) {
        self.memory.get_mut(&r.base).expect("a region made here").fill(0);
    }

    fn pid(&self) -> u32 {
        0x77
    }
}
