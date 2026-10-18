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

//! Context regions and their dwords, for the context tests.

use crate::dma::DmaPool;

pub(super) fn region(bytes: u64) -> crate::dma::DmaRegion {
    let r = DmaPool::new(1, 1).alloc(bytes).expect("region");
    r.zero();
    r
}

pub(super) fn dw(r: &crate::dma::DmaRegion, byte: usize) -> u32 {
    // SAFETY: the region is a live grant and `byte` is inside it.
    unsafe { (r.as_mut_ptr::<u8>().add(byte) as *const u32).read() }
}

pub(super) fn set_dw(r: &crate::dma::DmaRegion, byte: usize, v: u32) {
    // SAFETY: as for `dw`.
    unsafe { (r.as_mut_ptr::<u8>().add(byte) as *mut u32).write(v) }
}
