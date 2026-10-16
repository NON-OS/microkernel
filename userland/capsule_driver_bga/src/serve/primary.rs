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

//! OP_GET_PRIMARY_SURFACE: the surface's handle and geometry, laid out as
//! virtio-gpu lays them out. Only the process holding the compositor's name
//! is handed it (`compositor`), and maps it with MkSurfaceAttach and draws
//! in it. That is asked at each get_primary, so any other process is E_BUSY
//! whatever the kernel lets reach this service, and a compositor respawned
//! under a new pid takes the surface over once the old one is gone.

use nonos_libc::SURFACE_FORMAT_ARGB8888;

use super::compositor::compositor_pid;
use super::held::PRIMARY_RESOURCE_ID;
use super::wire::{E_BUSY, PRIMARY_LEN};
use crate::setup::Driver;

pub fn handle(d: &mut Driver, sender: u32) -> Result<[u8; PRIMARY_LEN], i32> {
    if compositor_pid() != Some(sender) {
        return Err(E_BUSY);
    }
    d.owner_pid = sender;
    let mut b = [0u8; PRIMARY_LEN];
    b[0..8].copy_from_slice(&d.surface.handle.to_le_bytes());
    b[8..12].copy_from_slice(&PRIMARY_RESOURCE_ID.to_le_bytes());
    b[12..16].copy_from_slice(&(d.width as u32).to_le_bytes());
    b[16..20].copy_from_slice(&(d.height as u32).to_le_bytes());
    b[20..24].copy_from_slice(&d.stride.to_le_bytes());
    b[24..28].copy_from_slice(&SURFACE_FORMAT_ARGB8888.to_le_bytes());
    Ok(b)
}
