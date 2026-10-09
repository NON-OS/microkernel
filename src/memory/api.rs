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

extern crate alloc;

use crate::memory::unified::{get_memory_system_stats, MemorySystemStats};

pub fn get_memory_stats() -> MemorySystemStats {
    get_memory_system_stats()
}

pub fn read_process_memory(pid: u32, addr: u64, buf: &mut [u8]) -> Result<usize, i32> {
    if buf.is_empty() {
        return Ok(0);
    }
    if crate::process::current_pid().ok_or(-3)? != pid {
        return Err(-1);
    }
    let pcb = crate::process::PROCESS_TABLE.find_by_pid(pid).ok_or(-3)?;
    /*
     * The copy runs after the VMA lock is dropped: it can fault a page in,
     * and a page-table change must not happen under that lock.
     */
    let vma_end = pcb
        .memory_state()
        .vmas
        .iter()
        .find(|vma| addr >= vma.start.as_u64() && addr < vma.end.as_u64())
        .map(|vma| vma.end.as_u64())
        .ok_or(-14)?;
    let copy_len = buf.len().min((vma_end - addr) as usize);
    crate::usercopy::copy_from_user(addr, &mut buf[..copy_len]).map_err(i32::from)?;
    Ok(copy_len)
}

pub fn get_process_vm_areas(pid: u32) -> alloc::vec::Vec<(u64, u64, u32)> {
    crate::process::PROCESS_TABLE
        .find_by_pid(pid)
        .map(|pcb| {
            pcb.memory_state()
                .vmas
                .iter()
                .map(|v| (v.start.as_u64(), v.end.as_u64(), v.flags as u32))
                .collect()
        })
        .unwrap_or_default()
}
