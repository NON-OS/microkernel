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

//! Taking pages away from a guest.

use crate::kernel_core::surface_registry::pin::held::Held;
use crate::kernel_core::surface_registry::pin::pin_guest::unpin_meeting;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::unmap_span_in_asid;
use crate::memory::paging::types::PagePermissions;
use crate::syscall::microkernel::errnos::ERRNO_INVAL;

use super::peer_guard::{in_user_half, supervised_asid, MAX_SPAN, PAGE};

fn span_ok(addr: u64, len: u64) -> bool {
    len != 0 && len <= MAX_SPAN && addr % PAGE == 0 && in_user_half(addr, len)
}

/// `MkPeerUnmap`: drop `[addr, addr + len)` from a guest the caller
/// supervises. A surface pinned over the span is unpinned first, and a frame
/// a reader of it still maps is left to the registry, freed once that reader
/// lets go, as an owner's munmap leaves it (`Held`).
pub fn sys_peer_unmap(pid: u64, addr: u64, len: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let (asid, _held) = match supervised_asid(caller, pid) {
        Ok(pair) => pair,
        Err(e) => return e,
    };
    if !span_ok(addr, len) {
        return ERRNO_INVAL;
    }
    let group = crate::hardware::broker::holder_of(pid as u32);
    unpin_meeting(group, addr, len);
    let mut held = Held::collect(group, addr, len);
    let perms = PagePermissions::READ | PagePermissions::USER;
    let pages = len.div_ceil(PAGE) as usize;
    let frames = unmap_span_in_asid(asid, VirtAddr::new(addr), pages, perms);
    for &(va, frame) in &frames {
        if !held.holds(va.as_u64(), frame) {
            let _ = crate::memory::frame_alloc::deallocate_frame(frame);
        }
    }
    held.settle();
    super::guest_stats::resident(pid as u32, 0, frames.len() as _);
    0
}
