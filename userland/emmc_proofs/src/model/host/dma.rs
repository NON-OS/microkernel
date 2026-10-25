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

//! Host memory as the descriptors name it: the bytes a read lands in and a
//! write takes from.

use super::sim::Sim;

impl Sim {
    pub(super) fn copy_in(&mut self, segs: &[(u64, usize)], bytes: &[u8]) {
        let mut o = 0;
        for &(addr, l) in segs {
            let p = self.ptr(addr, l);
            unsafe { std::ptr::copy_nonoverlapping(bytes[o..].as_ptr(), p, l) };
            o += l;
        }
    }

    pub(super) fn copy_out(&mut self, segs: &[(u64, usize)], len: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity(len);
        for &(addr, l) in segs {
            let p = self.ptr(addr, l);
            v.extend_from_slice(unsafe { std::slice::from_raw_parts(p, l) });
        }
        v
    }

    pub(super) fn ptr(&mut self, addr: u64, len: usize) -> *mut u8 {
        for &(bus, p, l) in &self.mem {
            if addr >= bus && addr + len as u64 <= bus + l as u64 {
                return (p + (addr - bus) as usize) as *mut u8;
            }
        }
        panic!("DMA to {addr:#x}+{len:#x} outside every region");
    }
}
