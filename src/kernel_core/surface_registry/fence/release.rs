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

//! A holder is done reading a surface: its count moves on and the project's
//! waiters wake. Only a process holding the surface attached may move it.

use super::super::attach_map::lookup;
use super::super::table::SLOTS;
use super::super::types::{decode_handle, RegistryError, SurfaceHandle};
use super::{count, waiters};

/// `holder` (a thread group) releases `handle`. Returns the new count.
pub fn release_fence(holder: u32, handle: SurfaceHandle) -> Result<u32, RegistryError> {
    if lookup(holder, handle).is_none() {
        return Err(RegistryError::NotOwner);
    }
    let (idx, epoch) = decode_handle(handle);
    {
        let slots = SLOTS.lock();
        let slot =
            slots.get(idx as usize).and_then(|s| s.as_ref()).ok_or(RegistryError::BadHandle)?;
        if slot.epoch != epoch {
            return Err(RegistryError::BadHandle);
        }
    }
    let n = count::bump(idx as usize).ok_or(RegistryError::BadHandle)?;
    wake_slot(idx);
    Ok(n)
}

/// Wake every waiter on slot `idx`, the lock dropped first: on a release,
/// and when the slot is freed, so a waiter finds its surface gone.
pub(in super::super) fn wake_slot(idx: u32) {
    for pid in waiters::take(idx) {
        crate::sched::wake_process(pid);
    }
}
