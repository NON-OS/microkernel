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

//! SCENE_SUBMIT and SCENE_REMOVE: a layer placed in a band, and layers
//! taken away.

use super::read::{wide, words};
use crate::protocol::{SCENE_REMOVE_REQ_LEN, SCENE_SUBMIT_REQ_LEN};
use crate::state::scene::Layer;

/// SCENE_REMOVE's flag: only the sender's layer in band z goes.
pub const BAND_ONLY: u32 = 1;

/// SCENE_SUBMIT's body: the surface handle u64, then x, y, width, height
/// and z, u32 each, and a padding word that must be zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placed {
    pub handle: u64,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub z: u32,
}

pub fn scene_submit(body: &[u8]) -> Option<Placed> {
    let [lo, hi, x, y, width, height, z, pad] = words::<8, SCENE_SUBMIT_REQ_LEN>(body)?;
    (pad == 0).then_some(Placed { handle: wide(lo, hi), x, y, width, height, z })
}

impl Placed {
    /// The layer for `owner` on a `w` by `h` canvas, or None for an empty
    /// one or one starting off the canvas. A window that runs past the edge
    /// is clipped rather than refused: the WM's display size can differ from
    /// the compositor's by an edge pixel, and a hard reject there left a
    /// window's first submit refused and blank.
    pub fn layer(&self, owner_pid: u32, w: u32, h: u32) -> Option<Layer> {
        let Placed { handle, x, y, width, height, z } = *self;
        if width == 0 || height == 0 || x >= w || y >= h {
            return None;
        }
        let (width, height) = (width.min(w - x), height.min(h - y));
        let (stack, in_use, miss_count, surface_handle) = (0, true, 0, handle);
        Some(Layer { owner_pid, surface_handle, x, y, width, height, z, stack, in_use, miss_count })
    }
}

/// SCENE_REMOVE's body: z, then flags.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Remove {
    pub z: u32,
    /// Only the layer in band z, not every layer the sender holds.
    pub band_only: bool,
}

pub fn scene_remove(body: &[u8]) -> Option<Remove> {
    let [z, flags] = words::<2, SCENE_REMOVE_REQ_LEN>(body)?;
    Some(Remove { z, band_only: flags & BAND_ONLY != 0 })
}
