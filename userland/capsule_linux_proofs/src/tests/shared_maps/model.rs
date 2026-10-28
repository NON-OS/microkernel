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

//! The protocol of echo.rs, back.rs and ends.rs, run over the shipped
//! rules on buffers that stand in for the guests and the family's copies,
//! beside the bytes a Linux page cache would hold (`truth`).

use super::support::{Entry, P};
use crate::linux::file::shared_maps::rules::admits;

pub const PATHS: [&[u8]; 2] = [b"/f", b"/g"];

pub struct World {
    pub truth: [Vec<u8>; 2],
    pub copy: [Vec<u8>; 2],
    pub mem: Vec<Vec<u8>>,
    pub table: Vec<Entry>,
}

pub fn ix(path: &[u8]) -> usize {
    PATHS.iter().position(|p| *p == path).expect("a path of the model")
}

impl World {
    pub fn new(pids: usize, pages: u64) -> World {
        let mem = vec![vec![0u8; (pages * P) as usize]; pids];
        World { truth: [vec![], vec![]], copy: [vec![], vec![]], mem, table: vec![] }
    }

    /// mmap as map_shared.rs makes one: admitted, room made, then filled.
    pub fn map(&mut self, e: Entry, fixed: bool) -> Result<(), i64> {
        admits(&self.table, &e, fixed)?;
        if fixed {
            self.unmap(e.pid, e.at, e.span)?;
        }
        let file = &self.copy[ix(&e.path)];
        for b in 0..e.span {
            let byte = file.get((e.off + b) as usize).copied().unwrap_or(0);
            self.mem[e.pid as usize][(e.at + b) as usize] = byte;
        }
        self.table.push(e);
        Ok(())
    }
}
