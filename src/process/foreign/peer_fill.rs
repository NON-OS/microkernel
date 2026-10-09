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
//! Kernel bytes into a guest: what `MkPeerCopy` does from a supervisor's
//! buffer, for bytes the kernel holds itself, such as a range of a data
//! volume file opened for the guest. Nothing passes through the
//! supervisor's own memory.

use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::manager::translate_in_asid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL};

use super::peer_guard::{in_user_half, supervised_asid, MAX_SPAN, PAGE};

/// Write `bytes` at `guest_addr` in the guest `pid` that `caller`
/// supervises, under the peer lock like every peer call. EFAULT when a page
/// of the span is not mapped; the pages before it may have been written.
pub fn fill_guest(caller: u32, pid: u64, guest_addr: u64, bytes: &[u8]) -> Result<(), i64> {
    let (asid, _held) = supervised_asid(caller, pid)?;
    let len = bytes.len() as u64;
    if len == 0 || len > MAX_SPAN || !in_user_half(guest_addr, len) {
        return Err(ERRNO_INVAL);
    }
    let mut done = 0u64;
    while done < len {
        let va = guest_addr + done;
        let page = va & !(PAGE - 1);
        let offset = va - page;
        let chunk = core::cmp::min(PAGE - offset, len - done);
        let phys = translate_in_asid(asid, VirtAddr::new(page)).ok_or(ERRNO_FAULT)?;
        let at = PhysAddr::new(phys.as_u64() + offset);
        let virt = crate::memory::unified::phys_to_virt(at).ok_or(ERRNO_FAULT)?;
        let from = &bytes[done as usize..(done + chunk) as usize];
        /*
         * SAFETY: eK@nonos.systems - `phys` came from the guest's own page
         * tables under the peer lock, and `offset + chunk` stays inside that
         * one frame, so the destination is a mapped page of the guest and
         * nothing else; `from` is a kernel slice of exactly `chunk` bytes.
         */
        unsafe {
            core::ptr::copy_nonoverlapping(from.as_ptr(), virt.as_u64() as *mut u8, from.len());
        }
        done += chunk;
    }
    Ok(())
}
