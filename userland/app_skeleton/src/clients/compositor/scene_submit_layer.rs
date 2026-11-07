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

//! OP_SCENE_SUBMIT_LAYER: one of the caller's layers, named by its surface
//! handle, for one of its windows. A caller holds one per window, dialog and
//! popup, each stacking and going on its own.

use crate::wire::{call_status, NCMP_MAGIC};

const OP: u16 = 0x001C;
const BODY_LEN: usize = 48;
const E_NOENT: i32 = -2;
const E_BUSY: i32 = -16;
const E_PERM: i32 = -1;

/// A layer as the compositor places it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyedLayer {
    pub surface_handle: u64,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    /// The band: the application band for a window, a dialog or a popup.
    pub z: u32,
    /// The window manager's id of the window this layer draws; never 0.
    pub window_id: u32,
    /// The handle of the caller's layer in band z this one takes the place
    /// of (a resized window's old surface), keeping its place in the stack;
    /// 0 for none.
    pub replaces: u64,
}

pub fn scene_submit_layer(
    port: u32,
    request_id: u32,
    layer: &KeyedLayer,
) -> Result<(), &'static str> {
    let mut body = [0u8; BODY_LEN];
    body[0..8].copy_from_slice(&layer.surface_handle.to_le_bytes());
    body[8..12].copy_from_slice(&layer.x.to_le_bytes());
    body[12..16].copy_from_slice(&layer.y.to_le_bytes());
    body[16..20].copy_from_slice(&layer.width.to_le_bytes());
    body[20..24].copy_from_slice(&layer.height.to_le_bytes());
    body[24..28].copy_from_slice(&layer.z.to_le_bytes());
    body[28..32].copy_from_slice(&layer.window_id.to_le_bytes());
    body[32..40].copy_from_slice(&layer.replaces.to_le_bytes());
    match call_status(port, NCMP_MAGIC, OP, request_id, &body)? {
        0 => Ok(()),
        E_NOENT => Err("compositor holds no such layer to replace"),
        E_BUSY => Err("compositor holds the caller's share of layers"),
        E_PERM => Err("compositor keeps that band for trusted sources"),
        _ => Err("compositor rejected scene_submit_layer"),
    }
}
