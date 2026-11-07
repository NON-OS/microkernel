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

//! Per-process kernel stack.
//!
//! Mapped in the kernel half by `memory::buddy_alloc::allocate_guarded`, with
//! an unmapped guard page below it, so a frame that runs past the bottom
//! faults on the spot (the page fault has its own IST stack) instead of
//! writing over the stack of whichever process lies below. The top is stored
//! in `pcb.kernel_stack_top` and consumed by the scheduler resume hook to load
//! TSS RSP0 before iretq-ing into a CPL=3 capsule. Distinct from the user
//! stack: a CPL=3 trap must not land on the same memory the capsule was using.

use crate::memory::addr::VirtAddr;
use crate::memory::buddy_alloc::{allocate_guarded, free_guarded, BuddyAllocError};
use crate::process::core::{Pid, PROCESS_TABLE};
use crate::process::userspace::constants::KERNEL_STACK_SIZE;
use core::sync::atomic::Ordering;

const PAGE_SIZE: usize = 4096;

/// The mapped pages of one kernel stack, without its guard.
pub(crate) const KERNEL_STACK_PAGES: usize = (KERNEL_STACK_SIZE + PAGE_SIZE - 1) / PAGE_SIZE;

#[derive(Debug, Clone, Copy)]
pub(crate) enum KernelStackError {
    NoSuchProcess,
    Allocation,
}

/// Allocate a 32 KiB kernel-only stack above a guard page and stash its top
/// on the PCB. Returns the top (16-byte aligned). The base is page-aligned
/// and `KERNEL_STACK_SIZE` a multiple of the page size, so
/// `top - KERNEL_STACK_SIZE` is the base `free_guarded` takes back.
pub(crate) fn allocate_kernel_stack(pid: Pid) -> Result<u64, KernelStackError> {
    let pcb = PROCESS_TABLE.find_by_pid(pid).ok_or(KernelStackError::NoSuchProcess)?;
    let base = allocate_guarded(KERNEL_STACK_PAGES).map_err(|e| {
        let mut l = crate::sys::serial::Line::new();
        l.str(b"[KSTACK] alloc err: ").str(buddy_err_name(e).as_bytes()).end();
        KernelStackError::Allocation
    })?;
    /*
     * The guard is what keeps an overrun on this stack; a stack whose guard
     * reads as mapped is not handed out.
     */
    let guard = VirtAddr::new(base.as_u64() - PAGE_SIZE as u64);
    if crate::memory::paging::manager::translate_address(guard).is_some() {
        let _ = free_guarded(base, KERNEL_STACK_PAGES);
        let mut l = crate::sys::serial::Line::new();
        l.str(b"[KSTACK] guard page mapped at ").hex(guard.as_u64()).str(b": stack refused").end();
        return Err(KernelStackError::Allocation);
    }
    let top = base.as_u64() + (KERNEL_STACK_PAGES * PAGE_SIZE) as u64;
    let top_aligned = top & !0xF;
    pcb.kernel_stack_top.store(top_aligned, Ordering::Release);
    Ok(top_aligned)
}

fn buddy_err_name(e: BuddyAllocError) -> &'static str {
    match e {
        BuddyAllocError::NotInitialized => "NotInitialized",
        BuddyAllocError::InvalidSize => "InvalidSize",
        BuddyAllocError::InvalidPageCount => "InvalidPageCount",
        BuddyAllocError::InvalidAlignment => "InvalidAlignment",
        BuddyAllocError::AllocationTooLarge => "AllocationTooLarge",
        BuddyAllocError::OutOfVirtualMemory => "OutOfVirtualMemory",
        BuddyAllocError::FrameAllocationFailed => "FrameAllocationFailed",
        BuddyAllocError::MappingFailed => "MappingFailed",
        BuddyAllocError::InvalidAddress => "InvalidAddress",
        BuddyAllocError::TranslationFailed => "TranslationFailed",
        BuddyAllocError::UnmapFailed => "UnmapFailed",
        BuddyAllocError::BlockOutOfRange => "BlockOutOfRange",
        BuddyAllocError::DoubleFree => "DoubleFree",
        BuddyAllocError::Overflow => "Overflow",
    }
}
