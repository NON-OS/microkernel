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

//! The prover's memory floor, held or refused before anything is asked of the
//! TPM. The prover peaks near 2.6 GB, so its heap is sized here, before the
//! first allocation, as the installer sizes its own: the whole floor when the
//! machine has it free with room to spare, else the default heap, in which the
//! window can still say why it stops. The free memory is `MkProcStat`'s
//! header, which every capsule may read.

use core::mem::size_of;

use nonos_libc::{heap_init_sized, mk_proc_stat, ProcStatEntry, ProcStatHeader};

/// The heap held while the window is open: the prover's peak, and the
/// registry it rebuilds first, freed before the proof starts.
pub const HEAP_MIB: u64 = 2816;
/// What the rest of the machine keeps beside it.
pub const SPARE_MIB: u64 = 256;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Memory {
    /// The prover's heap is held.
    Held,
    /// The machine has this much free, below the floor.
    Short(u64),
    /// The machine has the floor free, but the heap did not map.
    Unmapped,
    /// The kernel did not say how much is free.
    Unknown,
}

pub fn claim() -> Memory {
    let Some(free_kb) = free_kb() else {
        return Memory::Unknown;
    };
    let free_mib = free_kb / 1024;
    if free_mib < HEAP_MIB + SPARE_MIB {
        return Memory::Short(free_mib);
    }
    match heap_init_sized((HEAP_MIB as usize) << 20) {
        Ok(()) => Memory::Held,
        Err(_) => Memory::Unmapped,
    }
}

fn free_kb() -> Option<u64> {
    const LEN: usize = size_of::<ProcStatHeader>() + size_of::<ProcStatEntry>();
    let mut buf = [0u8; LEN];
    if mk_proc_stat(buf.as_mut_ptr(), 1) < 0 {
        return None;
    }
    /* SAFETY: the kernel wrote a whole header at the buffer's start; it is
     * read unaligned, as the buffer is bytes. */
    let h = unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) };
    Some(h.mem_free_kb)
}
