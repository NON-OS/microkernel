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

use crate::kernel_core::surface_registry::types::{RegistryError, SurfaceHandle};

use super::owned::with_owned;
use super::rule::Named;

/*
 * Name `receiver` the one process besides the owner that attaches `handle`
 * (L18c, K20); `write` maps its view writable. It must be a live process of
 * this machine: a name kept for a pid not yet made would let whoever got that
 * pid next attach. The receiver is named twice, as its thread group: in the
 * slot, checked before any record is read (`permit`), and as a right with its
 * write bit (`rights`), which the owner's release, the slot's end and the
 * receiver's exit take back. An attach needs both. Neither is a reference:
 * the receiver's attach takes one, so a share no longer keeps a surface
 * alive until its owner exits (K20).
 */
pub fn share_surface(
    owner_pid: u32,
    handle: SurfaceHandle,
    receiver: u32,
    write: bool,
) -> Result<SurfaceHandle, RegistryError> {
    let live = crate::process::with_process(receiver, |_| ()).is_some();
    if receiver == 0 || !live || crate::process::foreign::is_foreign(receiver) {
        return Err(RegistryError::InvalidArg);
    }
    // Resolved before the table lock: it takes other locks.
    let group = crate::hardware::broker::holder_of(receiver);
    with_owned(owner_pid, handle, |slot| {
        super::rights::record(handle, group, write)?;
        match slot.shares.name(group) {
            Named::Added | Named::Already => Ok(handle),
            Named::Full => Err(RegistryError::OutOfSlots),
            Named::Refused => Err(RegistryError::InvalidArg),
        }
    })
}
