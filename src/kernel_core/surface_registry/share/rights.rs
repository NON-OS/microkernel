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

//! The receivers each share named, and who may attach.

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use super::rights_attach::{attach, Attach};
use super::rights_rule::{grant, revoke_handle, Grant, Right};
use crate::kernel_core::surface_registry::pin::process_of::same_process;
use crate::kernel_core::surface_registry::table::SLOTS;
use crate::kernel_core::surface_registry::types::{decode_handle, RegistryError, SurfaceHandle};

static RIGHTS: Mutex<Vec<Right>> = Mutex::new(Vec::new());

/// Name `pid` a receiver of `handle`; true when it was not one already.
pub(super) fn record(handle: SurfaceHandle, pid: u32, write: bool) -> Result<bool, RegistryError> {
    match grant(&mut RIGHTS.lock(), Right { handle, pid, write }) {
        Grant::New => Ok(true),
        Grant::Already => Ok(false),
        Grant::Full => Err(RegistryError::OutOfSlots),
    }
}

/// Whose surface `handle` names now.
pub(super) fn owner(handle: SurfaceHandle) -> Option<u32> {
    let (idx, epoch) = decode_handle(handle);
    let slots = SLOTS.lock();
    let slot = slots.get(idx as usize)?.as_ref()?;
    (slot.epoch == epoch).then_some(slot.owner_pid)
}

/// Whether `pid` attaches `handle`, and how.
pub(super) fn may_attach(handle: SurfaceHandle, pid: u32) -> Attach {
    let owner = owner(handle);
    attach(&RIGHTS.lock(), owner, handle, pid, same_process)
}

/// The owner releasing `handle` takes back every receiver's right; a view
/// already mapped stays until that receiver releases it.
pub fn owner_releases(pid: u32, handle: SurfaceHandle) {
    if owner(handle).is_some_and(|o| same_process(o, pid)) {
        revoke(handle);
    }
}

/// No receiver attaches `handle` again: its owner released it, or it is gone.
pub fn revoke(handle: SurfaceHandle) {
    revoke_handle(&mut RIGHTS.lock(), handle);
}

/// `pid` exited: a later process given its pid inherits none of its rights.
pub fn revoke_pid(pid: u32) {
    super::rights_rule::revoke_pid(&mut RIGHTS.lock(), pid);
}
