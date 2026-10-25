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

//! The kernel as a VRAM driver sees it: a display pool of pages that
//! allocations come out of, a surface registry handing out handles, and
//! the shares each handle has. Released surfaces must be ones it handed
//! out, once each.

use std::collections::HashMap;

pub struct Pool {
    pub pages_left: usize,
    pub memory: HashMap<usize, Vec<u8>>,
    pub next_base: usize,
    pub surfaces: HashMap<u64, usize>,
    pub shares: Vec<(u64, u32)>,
    pub next_handle: u64,
    pub released: Vec<u64>,
    pub pid: u32,
}

impl Pool {
    pub fn new(pages: usize) -> Self {
        Pool {
            pages_left: pages,
            memory: HashMap::new(),
            next_base: 0x1000_0000,
            surfaces: HashMap::new(),
            shares: Vec::new(),
            next_handle: 1,
            released: Vec::new(),
            pid: 0x42,
        }
    }
}
