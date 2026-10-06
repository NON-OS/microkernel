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

//! A GGUF writer for tests: just enough to lay out headers, good and bad.

use alloc::vec::Vec;

#[derive(Default)]
pub struct Gguf {
    pub out: Vec<u8>,
}

impl Gguf {
    /// Magic, version and the two counts.
    pub fn header(version: u32, tensors: u64, keys: u64) -> Self {
        let mut g = Gguf::default();
        g.out.extend_from_slice(b"GGUF");
        g.u32(version).u64(tensors).u64(keys);
        g
    }
    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.out.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn u64(&mut self, v: u64) -> &mut Self {
        self.out.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn str(&mut self, s: &str) -> &mut Self {
        self.u64(s.len() as u64);
        self.out.extend_from_slice(s.as_bytes());
        self
    }
    /// A key holding a string value.
    pub fn kv_str(&mut self, k: &str, v: &str) -> &mut Self {
        self.str(k).u32(8).str(v)
    }
    /// A key holding a u32 value.
    pub fn kv_u32(&mut self, k: &str, v: u32) -> &mut Self {
        self.str(k).u32(4).u32(v)
    }
    /// A tensor description.
    pub fn tensor(&mut self, name: &str, shape: &[u64], ty: u32, offset: u64) -> &mut Self {
        self.str(name).u32(shape.len() as u32);
        for d in shape {
            self.u64(*d);
        }
        self.u32(ty).u64(offset)
    }
    /// Pad to `align`, then `bytes` of tensor data.
    pub fn data(&mut self, align: usize, bytes: usize) -> &mut Self {
        while self.out.len() % align != 0 {
            self.out.push(0);
        }
        self.out.resize(self.out.len() + bytes, 0x5a);
        self
    }
}
