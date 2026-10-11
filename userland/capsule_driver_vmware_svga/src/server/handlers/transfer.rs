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

//! OP_TRANSFER_TO_HOST: a rectangle of the surface copied into the visible
//! framebuffer, where the host reads it. `offset` must be where the
//! rectangle starts in the surface, as virtio-gpu's backing offset is.

use super::rect::rect_at;
use crate::constants::mode::{BYTES_PER_PIXEL, PRIMARY_RESOURCE};
use crate::protocol::{le_u32, E_INVAL};
use crate::screen::copy_rect;
use crate::setup::Driver;

const BODY_LEN: usize = 32;

pub fn transfer(d: &mut Driver, body: &[u8]) -> i32 {
    if body.len() != BODY_LEN || le_u32(body, 0) != Some(PRIMARY_RESOURCE) {
        return E_INVAL;
    }
    let Some(r) = rect_at(d, body, 4) else { return E_INVAL };
    let offset = u64::from_le_bytes(body[24..32].try_into().unwrap_or([0xFF; 8]));
    let start = r.y as u64 * d.surface.stride as u64 + (r.x * BYTES_PER_PIXEL) as u64;
    if offset != start {
        return E_INVAL;
    }
    // SAFETY: the surface and the framebuffer are this capsule's live
    // mappings, each `stride * height` long, and `rect_at` fit `r` to both
    // (they share the mode's width and height).
    unsafe { copy_rect(&d.surface, &d.fb, r) };
    0
}
