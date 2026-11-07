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
//! A surface flipped in with only the part of it that changed repainted:
//! the shell's layer showing `on` shows `handle` from now on.

use super::present_body::{present_layer_body, OP_PRESENT_LAYER};
use super::wire::call;

pub fn push_present_layer(
    compositor_port: u32,
    request_id: u32,
    (handle, on): (u64, u64),
    z: u32,
    area: (u32, u32, u32, u32),
    opaque: bool,
) -> Result<(), &'static str> {
    let body = present_layer_body((handle, on), z, area, opaque);
    let status = call(compositor_port, OP_PRESENT_LAYER, request_id, &body)?;
    if status != 0 {
        return Err("compositor rejected present_layer");
    }
    Ok(())
}
