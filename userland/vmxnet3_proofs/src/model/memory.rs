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

//! The device's view of memory: 64-bit bus addresses, each inside a region
//! the test granted. A DMA outside every region panics, which is the model
//! of a remapping unit faulting the access, and fails the test that made it.

pub struct Region {
    pub dev: u64,
    pub host: *mut u8,
    pub len: usize,
}

#[derive(Default)]
pub struct Memory {
    pub regions: Vec<Region>,
}

impl Memory {
    fn at(&self, dev: u64, len: usize) -> *mut u8 {
        for r in &self.regions {
            let off = dev.wrapping_sub(r.dev) as usize;
            if dev >= r.dev && off + len <= r.len {
                // SAFETY: inside a live host allocation the test granted.
                return unsafe { r.host.add(off) };
            }
        }
        panic!("part DMA outside every grant: {dev:#x}+{len}");
    }

    pub fn read(&self, dev: u64, out: &mut [u8]) {
        let p = self.at(dev, out.len());
        // SAFETY: `at` checked the whole range.
        unsafe { std::ptr::copy_nonoverlapping(p, out.as_mut_ptr(), out.len()) };
    }

    pub fn write(&self, dev: u64, data: &[u8]) {
        let p = self.at(dev, data.len());
        // SAFETY: `at` checked the whole range.
        unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), p, data.len()) };
    }

    pub fn u64(&self, dev: u64) -> u64 {
        let mut b = [0u8; 8];
        self.read(dev, &mut b);
        u64::from_le_bytes(b)
    }

    pub fn u32(&self, dev: u64) -> u32 {
        let mut b = [0u8; 4];
        self.read(dev, &mut b);
        u32::from_le_bytes(b)
    }

    pub fn set_u8(&self, dev: u64, v: u8) {
        self.write(dev, &[v]);
    }

    pub fn set_u32(&self, dev: u64, v: u32) {
        self.write(dev, &v.to_le_bytes());
    }
}
