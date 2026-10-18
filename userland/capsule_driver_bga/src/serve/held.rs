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

//! Who may use the primary resource, checked in virtio-gpu's order: a
//! resource that is not the primary is E_INVAL, a primary another process
//! was handed is E_BUSY. Until get_primary hands it to the compositor it is
//! nobody's, and every op on it is E_BUSY.

use super::wire::{E_BUSY, E_INVAL};
use crate::setup::Driver;

/// The one resource this driver has: the surface, scanned out whole.
pub const PRIMARY_RESOURCE_ID: u32 = 1;

pub fn held(d: &Driver, sender: u32, resource_id: u32) -> Result<(), i32> {
    if resource_id != PRIMARY_RESOURCE_ID {
        return Err(E_INVAL);
    }
    if d.owner_pid == 0 || d.owner_pid != sender {
        return Err(E_BUSY);
    }
    Ok(())
}
