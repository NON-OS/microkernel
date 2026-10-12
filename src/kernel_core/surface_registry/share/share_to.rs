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

//! Sharing a surface with one named process (L18c).

use crate::kernel_core::surface_registry::types::{RegistryError, SurfaceHandle};

use super::owned::with_owned;
use super::rule::Named;

/*
 * `receiver_pid`'s thread group may attach `handle` from now on. The pid must
 * name a live process of this machine: a name kept for a pid not yet made
 * would let whoever got that pid next attach. The reference taken matches
 * the older share's, so a caller moving to this call keeps its accounting.
 */
pub fn share_surface_to(
    owner_pid: u32,
    handle: SurfaceHandle,
    receiver_pid: u32,
) -> Result<SurfaceHandle, RegistryError> {
    let live = crate::process::with_process(receiver_pid, |_| ()).is_some();
    if receiver_pid == 0 || !live || crate::process::foreign::is_foreign(receiver_pid) {
        return Err(RegistryError::InvalidArg);
    }
    let receiver = crate::hardware::broker::holder_of(receiver_pid);
    with_owned(owner_pid, handle, |slot| {
        let refcount = slot.refcount.checked_add(1).ok_or(RegistryError::InvalidArg)?;
        match slot.shares.name(receiver) {
            Named::Added | Named::Already => {
                slot.refcount = refcount;
                Ok(handle)
            }
            Named::Full => Err(RegistryError::OutOfSlots),
            Named::Refused => Err(RegistryError::InvalidArg),
        }
    })
}
