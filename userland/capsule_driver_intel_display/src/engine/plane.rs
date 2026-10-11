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

//! Pointing the primary plane at a surface of the pipe's own size. Every
//! plane register written here is double-buffered: PLANE_SURF goes last
//! and arms them all, so the picture changes whole at one vertical blank
//! (SKL PRM Vol 2c, PLANE_SURF, "double buffered" in each register).

use super::access::Mmio;
use super::error::Error;
use super::pipe::Pipe;
use super::plane_read::TILING_MASK;
use super::plane_read::{ENABLE, FORMAT_MASK, FORMAT_RGB8888, ORDER_RGBX, ROTATION_MASK};
use super::plane_regs as r;

/// A linear surface's base: 256 KiB aligned (the PRM's PLANE_SURF note
/// for linear memory; unverified, and stricter than 4 KiB either way).
pub const SURF_ALIGN: u32 = 256 * 1024;
/// PLANE_STRIDE holds 10 bits of 64 byte units for linear.
pub const MAX_STRIDE: u32 = 1023 * 64;

/// Program plane 1 of `p` for a linear ARGB8888 surface at graphics
/// address `surf`, keeping the CTL bits (gamma, CSC) firmware set.
pub fn program(
    m: &mut dyn Mmio,
    p: Pipe,
    surf: u32,
    stride: u32,
    size: (u32, u32),
    old_ctl: u32,
) -> Result<(), Error> {
    let (w, h) = size;
    let stride_ok = stride.is_multiple_of(64) && stride >= w * 4 && stride <= MAX_STRIDE;
    if !surf.is_multiple_of(SURF_ALIGN)
        || !stride_ok
        || !(1..=8192).contains(&w)
        || !(1..=4096).contains(&h)
    {
        return Err(Error::Argument);
    }
    let keep = old_ctl & !(FORMAT_MASK | ORDER_RGBX | TILING_MASK | ROTATION_MASK);
    m.write(r::plane_stride(p, 1), stride / 64);
    m.write(r::plane_pos(p, 1), 0);
    m.write(r::plane_offset(p, 1), 0);
    m.write(r::plane_size(p, 1), ((h - 1) << 16) | (w - 1));
    m.write(r::plane_ctl(p, 1), keep | ENABLE | FORMAT_RGB8888);
    m.write(r::plane_surf(p, 1), surf);
    Ok(())
}
