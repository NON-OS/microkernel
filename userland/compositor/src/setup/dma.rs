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
//! Whether this boot runs with device DMA unconfined: the no-IOMMU posture,
//! which the kernel marks in MkProcStat's header (`BOOT_DMA_UNCONFINED`).
//! It is read once, when a context is built; a boot's posture never
//! changes. A kernel that will not answer is taken as confined, as an
//! older kernel's zero reads.

use core::mem::size_of;

use nonos_libc::procstat_header::BOOT_DMA_UNCONFINED;
use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

const LEN: usize = size_of::<ProcStatHeader>() + size_of::<ProcStatEntry>();

pub fn unconfined() -> bool {
    let mut buf = [0u8; LEN];
    if mk_proc_stat(buf.as_mut_ptr(), 1) < 0 {
        return false;
    }
    // The buffer starts with a whole header, read unaligned.
    let h = unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) };
    h.boot_flags & BOOT_DMA_UNCONFINED != 0
}
