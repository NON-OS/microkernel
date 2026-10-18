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

use core::ptr::{self, addr_of_mut};
use core::sync::atomic::{fence, Ordering};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Descriptor {
    pub opts1: u32,
    pub opts2: u32,
    pub addr_lo: u32,
    pub addr_hi: u32,
}

/// Read descriptor `idx` of the ring at `base`.
///
/// # Safety
///
/// `base` must be the virtual address of a ring the caller mapped, holding at
/// least `idx + 1` descriptors, and the part may be writing the same entry
/// through DMA; the read is volatile for that reason.
pub unsafe fn desc(base: u64, idx: usize) -> Descriptor {
    ptr::read_volatile((base as *const Descriptor).add(idx))
}

/// Write descriptor `idx` of the ring at `base`.
///
/// # Safety
///
/// `base` must be the virtual address of a ring the caller mapped, holding at
/// least `idx + 1` descriptors, and the entry must not be one the part
/// currently owns, since the part reads it through DMA without a lock.
pub unsafe fn desc_mut(base: u64, idx: usize, value: Descriptor) {
    ptr::write_volatile((base as *mut Descriptor).add(idx), value);
}

/// Hand descriptor `idx` of the ring at `base` to the part: the address and
/// opts2 first, then a release fence, then opts1, which carries OWN, in a
/// store of its own, as Linux orders it with dma_wmb(). A part that sees OWN
/// then finds the address and the frame bytes already in place; one store of
/// the whole descriptor leaves the order of its parts unspecified.
///
/// # Safety
///
/// As `desc_mut`.
pub unsafe fn publish(base: u64, idx: usize, value: Descriptor) {
    let d = (base as *mut Descriptor).add(idx);
    ptr::write_volatile(addr_of_mut!((*d).addr_lo), value.addr_lo);
    ptr::write_volatile(addr_of_mut!((*d).addr_hi), value.addr_hi);
    ptr::write_volatile(addr_of_mut!((*d).opts2), value.opts2);
    fence(Ordering::Release);
    ptr::write_volatile(addr_of_mut!((*d).opts1), value.opts1);
}
