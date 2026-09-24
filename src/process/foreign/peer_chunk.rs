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

//! One page of a peer copy, and the caller-side validation that precedes
//! the whole transfer.

use crate::memory::addr::PhysAddr;

/// Check the caller's own buffer for the direction being copied.
pub(super) fn validate(buf: u64, len: u64, writing: bool) -> Result<(), ()> {
    let check = if writing {
        crate::usercopy::validate_user_read(buf, len as usize)
    } else {
        crate::usercopy::validate_user_write(buf, len as usize)
    };
    check.map_err(|_| ())
}

/// Copy within one guest frame, in whichever direction `writing` names.
pub(super) fn chunk_copy(
    phys: PhysAddr,
    offset: u64,
    buf: u64,
    len: u64,
    writing: bool,
) -> Result<(), ()> {
    let at = PhysAddr::new(phys.as_u64() + offset);
    let virt = crate::memory::unified::phys_to_virt(at).ok_or(())?;
    // SAFETY: eK@nonos.systems - `phys` came from the guest's own page
    // tables and `offset + len` stays inside that one frame, so this slice
    // is a mapped page of the guest and nothing else. The caller's buffer
    // was validated for this direction before the loop began.
    let guest = unsafe { core::slice::from_raw_parts_mut(virt.as_u64() as *mut u8, len as usize) };
    if writing {
        crate::usercopy::copy_from_user(buf, guest).map_err(|_| ())
    } else {
        crate::usercopy::copy_to_user(buf, guest).map_err(|_| ())
    }
}
