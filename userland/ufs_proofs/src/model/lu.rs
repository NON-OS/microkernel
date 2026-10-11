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

//! A logical unit: its size, names, how it comes ready, and its blocks.

use std::collections::HashMap;

pub struct Lu {
    pub lun: u8,
    pub block_len: u32,
    pub blocks: u64,
    /// TEST UNIT READY answers NOT READY this many times first.
    pub not_ready: u32,
    /// The first command after power-on gets UNIT ATTENTION.
    pub unit_attention: bool,
    pub write_protect: bool,
    pub vendor: &'static [u8; 8],
    pub product: &'static [u8; 16],
    pub serial: Option<&'static [u8]>,
    pub data: HashMap<u64, Vec<u8>>,
}

impl Lu {
    pub fn new(lun: u8, block_len: u32, blocks: u64) -> Self {
        Self {
            lun,
            block_len,
            blocks,
            not_ready: 0,
            unit_attention: true,
            write_protect: false,
            vendor: b"QEMU    ",
            product: b"QEMU HARDDISK   ",
            serial: Some(b"ufs-lu-0"),
            data: HashMap::new(),
        }
    }

    pub fn block(&self, lba: u64) -> Vec<u8> {
        self.data.get(&lba).cloned().unwrap_or_else(|| vec![0; self.block_len as usize])
    }
}
