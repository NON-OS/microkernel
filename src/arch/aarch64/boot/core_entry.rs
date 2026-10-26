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

//! Leaving the aarch64 bring-up for the path every architecture shares.

use super::info::BootInfo;
use crate::boot::handoff::KernelHandoff;

pub(super) fn enter(info: &BootInfo) -> ! {
    // Arm the bootstrap heap before any shared code runs. It hands out of a
    // static region, so it needs nothing but the MMU that init brought up.
    // x86_64 does this inside its own early setup, which this path does not
    // share; without it the first allocation is the frame allocator's bitmap
    // and it fails with the heap reporting no memory at all.
    crate::memory::heap::manager::init_bootstrap();
    crate::log::init();

    let handoff = KernelHandoff::from_aarch64(info);
    crate::kernel_core::microkernel_init(&handoff);
    crate::kernel_core::microkernel_main()
}
