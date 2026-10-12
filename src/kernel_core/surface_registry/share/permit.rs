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
 * receiver's thread group, or shared it the older way and the receiver
 * serves the compositor now. A refusal reads as a bad handle, the same as a
 * dead one, so a guesser cannot learn which slots are live. Groups are
 * resolved before the table lock: both take other locks.
 */
pub(super) fn permit(receiver_pid: u32, handle: SurfaceHandle) -> Result<(), RegistryError> {
    let receiver = crate::hardware::broker::holder_of(receiver_pid);
    let compositor = crate::services::registry::lookup_service("compositor")
        .map(|ep| crate::hardware::broker::holder_of(ep.pid));
    let (idx, epoch) = decode_handle(handle);
    let slots = SLOTS.lock();
    let slot = slots.get(idx as usize).and_then(|s| s.as_ref()).ok_or(RegistryError::BadHandle)?;
    if slot.epoch != epoch || !slot.shares.may_attach(receiver, compositor) {
        return Err(RegistryError::BadHandle);
    }
    Ok(())
}
