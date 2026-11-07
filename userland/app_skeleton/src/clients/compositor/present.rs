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

//! OP_PRESENT: the caller's layer shows another of its swapchain buffers.
//! The body is `nonos_surface::Present::encode`, 32 bytes.

use crate::swap::{
    OP_PRESENT, OP_PRESENT_LAYER, OP_PRESENT_LAYER_PARTS, PRESENT_LAYER_LEN,
    PRESENT_LAYER_PARTS_LEN, PRESENT_LEN,
};
use crate::wire::{call_status, NCMP_MAGIC};

pub fn present(port: u32, request_id: u32, body: &[u8; PRESENT_LEN]) -> Result<(), &'static str> {
    let status = call_status(port, NCMP_MAGIC, OP_PRESENT, request_id, body)?;
    if status != 0 {
        return Err("compositor rejected present");
    }
    Ok(())
}

/// OP_PRESENT_LAYER: the same for the caller's layer that shows the handle
/// at the body's end (`swap::on_layer`), one of several in its band.
pub fn present_layer(
    port: u32,
    request_id: u32,
    body: &[u8; PRESENT_LAYER_LEN],
) -> Result<(), &'static str> {
    let status = call_status(port, NCMP_MAGIC, OP_PRESENT_LAYER, request_id, body)?;
    if status != 0 {
        return Err("compositor rejected present");
    }
    Ok(())
}

/// OP_PRESENT_LAYER_PARTS: OP_PRESENT_LAYER repainting only the boxes the
/// body's parts block names (`swap::with_parts`).
pub fn present_layer_parts(
    port: u32,
    request_id: u32,
    body: &[u8; PRESENT_LAYER_PARTS_LEN],
) -> Result<(), &'static str> {
    let status = call_status(port, NCMP_MAGIC, OP_PRESENT_LAYER_PARTS, request_id, body)?;
    if status != 0 {
        return Err("compositor rejected present");
    }
    Ok(())
}
