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

//! The pool as the server's platform.

use super::pool::Pool;
use crate::platform::{Platform, Region};
use nonos_gfx_wire::errno::ENOMEM;
use nonos_gfx_wire::Errno;

impl Platform for Pool {
    fn alloc(&mut self, len: usize) -> Result<Region, Errno> {
        let pages = len.div_ceil(4096);
        if pages > self.pages_left {
            return Err(ENOMEM);
        }
        self.pages_left -= pages;
        let base = self.next_base;
        self.next_base += pages * 4096;
        self.memory.insert(base, vec![0; pages * 4096]);
        Ok(Region { base, len: pages * 4096 })
    }
    fn register(&mut self, r: Region, _w: u32, _h: u32, _stride: u32) -> Result<u64, Errno> {
        let h = self.next_handle;
        self.next_handle += 1;
        self.surfaces.insert(h, r.base);
        Ok(h)
    }
    fn share(&mut self, handle: u64, pid: u32) -> Result<(), Errno> {
        assert!(self.surfaces.contains_key(&handle), "sharing an unregistered surface");
        self.shares.push((handle, pid));
        Ok(())
    }
    fn release(&mut self, handle: Option<u64>, r: Region) {
        if let Some(h) = handle {
            assert_eq!(self.surfaces.remove(&h), Some(r.base), "released twice or never made");
            self.released.push(h);
        }
        let m = self.memory.remove(&r.base).expect("freed twice");
        self.pages_left += m.len() / 4096;
    }
    fn bytes(&self, r: Region) -> &[u8] {
        &self.memory[&r.base]
    }
    fn pid(&self) -> u32 {
        self.pid
    }
}
