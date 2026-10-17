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

//! A display pool and registry, the platform half of the server.

use nonos_gfx_serve::{Platform, Region};
use nonos_gfx_wire::Errno;
use std::collections::HashMap;

#[derive(Default)]
pub struct Pool {
    pub memory: HashMap<usize, Vec<u8>>,
    pub next: usize,
    pub handles: u64,
}

impl Platform for Pool {
    fn alloc(&mut self, len: usize) -> Result<Region, Errno> {
        self.next += 0x100_0000;
        self.memory.insert(self.next, vec![0; len]);
        Ok(Region { base: self.next, len })
    }
    fn register(&mut self, _r: Region, _w: u32, _h: u32, _s: u32) -> Result<u64, Errno> {
        self.handles += 1;
        Ok(self.handles)
    }
    fn share(&mut self, _h: u64, _pid: u32) -> Result<(), Errno> {
        Ok(())
    }
    fn release(&mut self, _h: Option<u64>, r: Region) {
        self.memory.remove(&r.base);
    }
    fn bytes(&self, r: Region) -> &[u8] {
        &self.memory[&r.base]
    }
    fn scrub(&mut self, r: Region) {
        self.memory.get_mut(&r.base).expect("a region made here").fill(0);
    }
    fn pid(&self) -> u32 {
        9
    }
}
