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

//! The attach check (L18c): only a receiver the owner shared with.

use crate::kernel_core::surface_registry::table::SLOTS;
use crate::kernel_core::surface_registry::types::{decode_handle, RegistryError, SurfaceHandle};

/*
 * Refuse `receiver_pid` unless `handle`'s owner shared it with the
 * receiver's thread group. Every share names its receiver now (K20): none
 * stands for whoever serves the compositor. A refusal is the same whether or
 * not the handle names a surface, the EPERM a right refused gives too, so a
 * guesser cannot learn which slots are live. The group is resolved before the
 * table lock: it takes other locks.
 */
pub(super) fn permit(receiver_pid: u32, handle: SurfaceHandle) -> Result<(), RegistryError> {
    let receiver = crate::hardware::broker::holder_of(receiver_pid);
    let (idx, epoch) = decode_handle(handle);
    let slots = SLOTS.lock();
    let slot = slots.get(idx as usize).and_then(|s| s.as_ref());
    match slot {
        Some(s) if s.epoch == epoch && s.shares.may_attach(receiver) => Ok(()),
        _ => Err(RegistryError::NotOwner),
    }
}
