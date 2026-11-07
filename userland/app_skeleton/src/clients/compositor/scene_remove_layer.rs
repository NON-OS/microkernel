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

//! OP_SCENE_REMOVE_LAYER: one of the caller's keyed layers goes; every other
//! layer it holds stays as it is.

use crate::wire::{call_status, NCMP_MAGIC};

const OP: u16 = 0x001D;
const BODY_LEN: usize = 16;

pub fn scene_remove_layer(
    port: u32,
    request_id: u32,
    surface_handle: u64,
    z: u32,
) -> Result<(), &'static str> {
    let mut body = [0u8; BODY_LEN];
    body[0..8].copy_from_slice(&surface_handle.to_le_bytes());
    body[8..12].copy_from_slice(&z.to_le_bytes());
    let status = call_status(port, NCMP_MAGIC, OP, request_id, &body)?;
    if status != 0 {
        return Err("compositor rejected scene_remove_layer");
    }
    Ok(())
}
