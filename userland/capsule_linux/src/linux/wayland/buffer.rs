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

//! `wl_shm_pool.create_buffer`: a rectangle inside a pool.

use crate::linux::guest::Guest;

use super::args::Args;
use super::object::Object;
use super::ops::{FORMAT_ARGB8888, FORMAT_XRGB8888};
use super::state::Buffer;

/// Bytes per pixel in both formats this capsule advertises.
const BPP: u64 = 4;

pub fn create_buffer(guest: &mut Guest, pool: u32, args: &mut Args<'_>) {
    let (Some(id), Some(offset), Some(width), Some(height), Some(stride), Some(format)) =
        (args.u32(), args.u32(), args.u32(), args.u32(), args.u32(), args.u32())
    else {
        return;
    };
    /* Only the two formats shm.rs advertises; a client sends no other. */
    if format != FORMAT_ARGB8888 && format != FORMAT_XRGB8888 {
        return;
    }
    let Some(size) = guest.scene.pools.iter().find(|p| p.id == pool).map(|p| p.size) else {
        return;
    };
    if !fits(offset as u64, width as u64, height as u64, stride as u64, size) {
        return;
    }
    if !guest.objects.put(id, Object::Buffer) {
        return;
    }
    guest.scene.buffers.retain(|b| b.id != id);
    guest.scene.buffers.push(Buffer {
        id,
        pool,
        offset: offset as u64,
        width,
        height,
        stride,
        opaque: format == FORMAT_XRGB8888,
    });
}

/// Whether the rectangle lies wholly inside a pool of `size` bytes.
fn fits(offset: u64, width: u64, height: u64, stride: u64, size: u64) -> bool {
    if width == 0 || height == 0 {
        return false;
    }
    let Some(row) = width.checked_mul(BPP) else {
        return false;
    };
    if stride < row {
        return false;
    }
    let Some(span) = stride.checked_mul(height) else {
        return false;
    };
    matches!(offset.checked_add(span), Some(end) if end <= size)
}
