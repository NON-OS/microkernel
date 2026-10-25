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

//! DMA sync windows on riscv64. The base ISA has no cache maintenance; a part
//! whose devices do not snoop needs the Zicbom extension's cbo.clean and
//! cbo.flush, and a way to learn the CPU has them, neither of which is in this
//! tree yet. QEMU's virt board models no caches, so a full FENCE over memory
//! and I/O is all a sync needs there. A non-coherent buffer therefore gets no
//! maintenance here: no riscv64 driver is given DMA yet, since with no IOMMU
//! the kernel refuses bus mastering.

use core::sync::atomic::{compiler_fence, Ordering};

use crate::memory::addr::VirtAddr;
use crate::memory::dma::coherency::Coherency;
use crate::memory::dma::types::DmaDirection;

pub(crate) fn sync_for_device(
    _cpu_addr: VirtAddr,
    _size: usize,
    _direction: DmaDirection,
    _coherency: Coherency,
) {
    full_fence();
}

pub(crate) fn sync_for_cpu(
    _cpu_addr: VirtAddr,
    _size: usize,
    _direction: DmaDirection,
    _coherency: Coherency,
) {
    full_fence();
}

fn full_fence() {
    compiler_fence(Ordering::SeqCst);
    // SAFETY: a fence orders accesses and has no other effect.
    unsafe { core::arch::asm!("fence iorw, iorw", options(nostack, preserves_flags)) };
    compiler_fence(Ordering::SeqCst);
}
