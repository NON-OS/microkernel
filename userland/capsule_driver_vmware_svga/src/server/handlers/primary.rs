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

//! OP_GET_PRIMARY_SURFACE: the surface handle, its geometry and format.
//! The first caller becomes the only one that may draw; another is told
//! the surface is busy.

use super::super::respond;
use crate::constants::mode::{FORMAT_B8G8R8A8, PRIMARY_RESOURCE};
use crate::protocol::{Request, E_BUSY, HDR_LEN, STATUS_LEN};
use crate::setup::Driver;

const RESP_LEN: usize = 32;

pub fn primary(d: &mut Driver, sender: u32, req: &Request, tx: &mut [u8]) -> Option<i32> {
    if d.owner == 0 {
        d.owner = sender;
    } else if d.owner != sender {
        return Some(E_BUSY);
    }
    let b = &mut tx[HDR_LEN + STATUS_LEN..HDR_LEN + STATUS_LEN + RESP_LEN];
    b[0..8].copy_from_slice(&d.handle.to_le_bytes());
    b[8..12].copy_from_slice(&PRIMARY_RESOURCE.to_le_bytes());
    b[12..16].copy_from_slice(&d.surface.width.to_le_bytes());
    b[16..20].copy_from_slice(&d.surface.height.to_le_bytes());
    b[20..24].copy_from_slice(&d.surface.stride.to_le_bytes());
    b[24..28].copy_from_slice(&FORMAT_B8G8R8A8.to_le_bytes());
    b[28..32].fill(0);
    respond::payload(sender, req, RESP_LEN, tx);
    None
}
