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

use super::shm_fit::fits;
/// wl_shm.error: invalid_format 0, invalid_stride 2.
const INVALID_FORMAT: u32 = 0;
const INVALID_STRIDE: u32 = 2;

pub fn create_buffer(guest: &mut Guest, pool: u32, args: &mut Args<'_>) {
    let (Some(id), Some(offset), Some(width), Some(height), Some(stride), Some(format)) =
        (args.u32(), args.u32(), args.u32(), args.u32(), args.u32(), args.u32())
    else {
        return;
    };
    /* Only the two formats shm.rs advertises; a client sends no other. */
    if format != FORMAT_ARGB8888 && format != FORMAT_XRGB8888 {
        return refused(guest, pool, INVALID_FORMAT, "a format wl_shm did not offer");
    }
    let Some(size) = guest.scene.pools.iter().find(|p| p.id == pool).map(|p| bound(guest, p))
    else {
        return;
    };
    if !fits(offset as u64, width as u64, height as u64, stride as u64, size) {
        return refused(guest, pool, INVALID_STRIDE, "a buffer that does not fit its pool");
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

/// Made nothing: the client is told, as libwayland's shm tells it, rather
/// than failing later on a buffer it thinks it has.
fn refused(guest: &mut Guest, pool: u32, code: u32, message: &str) {
    super::display_error::protocol(guest, pool, code, message);
}

/// How much of pool `p` a buffer may be made in: the size its client gave
/// it, and no further than the client has its memfd mapped now. A pool not
/// mapped now (its client let the mapping go before this request was read)
/// is held to its size alone: its buffers are made, so the client may go on
/// naming them, but none is read until the memfd is mapped again
/// (present/source.rs).
fn bound(guest: &Guest, p: &super::state::Pool) -> u64 {
    let mapped = crate::linux::guest::memfd_spans::reach(&guest.memfd_spans, p.memfd);
    mapped.map_or(p.size, |reach| reach.min(p.size))
}
