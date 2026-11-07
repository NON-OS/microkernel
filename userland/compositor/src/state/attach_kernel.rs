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

use nonos_libc::{mk_surface_attach, mk_surface_release, mk_syscall_raw, SurfaceDescriptor};

use super::answer_rule::Owners;
use super::attach::SurfaceKernel;
use crate::sw_blitter::Surface;

/// MkThreadGroup, as capsule_acpi asks it (server/caller.rs).
const THREAD_GROUP: i64 = u32::from_le_bytes(*b"MTGR") as i64;

/// The real surface registry, through its two syscalls.
pub struct Kernel;

impl SurfaceKernel for Kernel {
    fn attach(&mut self, handle: u64) -> Option<Surface> {
        let mut desc = SurfaceDescriptor::default();
        let rc = mk_surface_attach(handle, &mut desc);
        if rc <= 0 {
            return None;
        }
        Some(Surface {
            base_va: rc as u64,
            stride: desc.stride,
            width: desc.width,
            height: desc.height,
            byte_len: desc.byte_len,
        })
    }

    fn release(&mut self, handle: u64) -> bool {
        mk_surface_release(handle) >= 0
    }
}

impl Owners for Kernel {
    /// The descriptor of a view of `handle` names its owner or, for one
    /// pinned from a guest, the guest's supervisor. A view the cache holds
    /// already is answered from the kernel's record, with no new mapping.
    fn owner(&mut self, handle: u64) -> Option<u32> {
        let mut desc = SurfaceDescriptor::default();
        (mk_surface_attach(handle, &mut desc) > 0).then_some(desc.owner_pid)
    }

    fn release(&mut self, handle: u64) {
        let _ = mk_surface_release(handle);
    }

    /// MkThreadGroup: the sender itself when the kernel names no group.
    fn process_of(&mut self, sender: u32) -> u32 {
        let g = mk_syscall_raw(THREAD_GROUP, [u64::from(sender), 0, 0, 0, 0, 0]);
        u32::try_from(g).ok().filter(|&g| g != 0).unwrap_or(sender)
    }
}
