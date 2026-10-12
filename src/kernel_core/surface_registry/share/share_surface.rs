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

/// The older share, which names no receiver: the compositor may attach.
pub fn share_surface(
    owner_pid: u32,
    handle: SurfaceHandle,
) -> Result<SurfaceHandle, RegistryError> {
    with_owned(owner_pid, handle, |slot| {
        slot.refcount = slot.refcount.checked_add(1).ok_or(RegistryError::InvalidArg)?;
        slot.shares.allow_compositor();
        Ok(handle)
    })
}
