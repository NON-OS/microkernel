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

use alloc::vec::Vec;

use crate::kernel_core::surface_registry::SurfaceDescriptor;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::manager::api::translate_address;
use crate::syscall::dispatch::util::errno;
use crate::syscall::SyscallResult;
use crate::usercopy::validate_user_write;

use super::super::surface_ops::{EFAULT, EINVAL};

/// The frames behind the surface `desc` names, or the refusal.
pub(super) fn user_frames(desc: &SurfaceDescriptor) -> Result<Vec<PhysAddr>, SyscallResult> {
    // Bound the user-supplied length before deriving a page count: an
    // unbounded byte_len would force a huge Vec allocation and a near-infinite
    // translate loop in the kernel. 64 MiB covers any realistic surface
    // (4K BGRA is ~33 MiB) while capping the work at 16384 pages.
    const MAX_SURFACE_BYTES: u64 = 64 * 1024 * 1024;
    if desc.byte_len == 0 || desc.byte_len > MAX_SURFACE_BYTES || (desc.base_va & 0xFFF) != 0 {
        return Err(errno(EINVAL));
    }
    // The surface must be the caller's own user-writable memory. Validate the
    // whole range as user-writable before translating any address. Without
    // this, base_va could point into the higher-half directmap (present in
    // every capsule's CR3) and capture arbitrary kernel physical frames, which
    // attach would then map USER_RW. The walk rejects any page that is not
    // present, user-accessible, and writable, and bounds base_va+byte_len to
    // user space with checked arithmetic.
    if validate_user_write(desc.base_va, desc.byte_len as usize).is_err() {
        return Err(errno(EFAULT));
    }
    let pages = ((desc.byte_len as usize) + 4095) / 4096;
    let mut frames = Vec::with_capacity(pages);
    for i in 0..pages {
        let va = VirtAddr::new(desc.base_va + (i as u64) * 4096);
        let Some(pa) = translate_address(va) else {
            return Err(errno(EFAULT));
        };
        frames.push(PhysAddr::new(pa.as_u64() & !0xFFF));
    }
    Ok(frames)
}
