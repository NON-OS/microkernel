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

//! The band's plane: universal plane 2 over the primary, a small linear
//! surface of the driver's own holding the compositor's band while plane 1
//! scans a client's buffer out (SKL PRM Vol 2c, PLANE_* for plane 2,
//! PLANE_BUF_CFG and PLANE_WM per plane, 0x100 apart). Its display buffer
//! run is carved from the end of plane 1's, once, and plane 1's watermark
//! levels that no longer fit its smaller run are turned off. Its
//! registers latch with its own PLANE_SURF.

use super::access::Mmio;
use super::ddb::Run;
use super::error::Error;
use super::gen::PlaneKind;
use super::pipe::Pipe;
use super::plane::SURF_ALIGN;
use super::plane_read::ENABLE;
use super::plane_regs as r;
use super::wm::Level;
use super::wm_write::plane_buf_cfg;

pub const PLANE: u32 = 2;
/// The band image's row stride: 128 pixels, a multiple of 64 bytes.
pub const STRIDE: u32 = 512;
/// Its memory: 32 rows of 512 bytes, four GGTT entries.
pub const PAGES: u32 = 4;
pub const BYTES: usize = (STRIDE * 32) as usize;
/// The blocks the band's run takes from plane 1's (one 512-byte line
/// each, with room for the latency's lines).
pub const BLOCKS: u32 = 8;

fn plane2_buf_cfg(p: Pipe) -> u32 {
    plane_buf_cfg(p) + 0x100
}
fn plane_wm(p: Pipe, plane: u32, level: u32) -> u32 {
    0x7_0240 + 0x1000 * p.index() + 0x100 * (plane - 1) + 4 * level
}
fn plane_wm_trans(p: Pipe, plane: u32) -> u32 {
    0x7_0268 + 0x1000 * p.index() + 0x100 * (plane - 1)
}

/// Plane 1's run less its last `BLOCKS`, and the band's; None when plane
/// 1 would keep less than four times the band's share.
pub fn carve(run: Run) -> Option<(Run, Run)> {
    if run.len() < BLOCKS * 5 {
        return None;
    }
    let cut = run.end - BLOCKS;
    Some((Run { start: run.start, end: cut }, Run { start: cut, end: run.end }))
}

/// Plane 1's level registers for a run of `share` blocks: a level whose
/// blocks no longer fit is off, and every level above it.
pub fn trim(levels: [u32; 8], share: u32) -> [u32; 8] {
    let mut out = [0u32; 8];
    for (o, &l) in out.iter_mut().zip(levels.iter()) {
        let (on, blocks) = (l & (1 << 31) != 0, l & 0x3FF);
        if !on || blocks >= share {
            break;
        }
        *o = l;
    }
    out
}

/// Give plane 2 its run out of plane 1's, latched with plane 1's next
/// PLANE_SURF and plane 2's first. None of plane 1's own run changes when
/// firmware gave it too little to share.
pub fn carve_regs(m: &mut dyn Mmio, p: Pipe) -> Result<(), Error> {
    let (one, two) = carve(Run::of(m.read(plane_buf_cfg(p)))).ok_or(Error::Layout)?;
    let levels: [u32; 8] = core::array::from_fn(|l| m.read(plane_wm(p, 1, l as u32)));
    for (l, v) in trim(levels, one.len()).iter().enumerate() {
        m.write(plane_wm(p, 1, l as u32), *v);
    }
    m.write(plane_buf_cfg(p), one.cfg());
    m.write(plane2_buf_cfg(p), two.cfg());
    let level0 = Level { enabled: true, lines: 1, blocks: BLOCKS };
    m.write(plane_wm(p, PLANE, 0), level0.reg());
    for l in 1..8 {
        m.write(plane_wm(p, PLANE, l), 0);
    }
    m.write(plane_wm_trans(p, PLANE), 0);
    Ok(())
}

/// Whether plane 2 still holds a run of its own beside plane 1's, after a
/// mode set rewrote plane 1's; one that now overlaps it is taken away.
pub fn still_carved(m: &mut dyn Mmio, p: Pipe) -> bool {
    let (one, two) = (Run::of(m.read(plane_buf_cfg(p))), Run::of(m.read(plane2_buf_cfg(p))));
    let apart = two.end <= one.start || one.end <= two.start;
    if two.is_empty() || !apart {
        m.write(plane2_buf_cfg(p), 0);
        return false;
    }
    true
}

/// Show the band image at graphics address `surf`, `size` pixels at `at`
/// on the pipe, opaque, over plane 1.
pub fn show(
    m: &mut dyn Mmio,
    kind: PlaneKind,
    p: Pipe,
    surf: u32,
    at: (u32, u32),
    size: (u32, u32),
) -> Result<(), Error> {
    let (w, h) = size;
    let fits = (1..=STRIDE / 4).contains(&w) && (1..=32).contains(&h);
    if kind == PlaneKind::Primary || !surf.is_multiple_of(SURF_ALIGN) || !fits {
        return Err(Error::Argument);
    }
    m.write(r::plane_stride(p, PLANE), kind.stride_reg(STRIDE));
    m.write(r::plane_offset(p, PLANE), 0);
    m.write(r::plane_pos(p, PLANE), (at.1 << 16) | at.0);
    m.write(r::plane_size(p, PLANE), ((h - 1) << 16) | (w - 1));
    m.write(r::plane_ctl(p, PLANE), ENABLE | kind.format_argb());
    m.write(r::plane_surf(p, PLANE), surf);
    Ok(())
}

/// Turn plane 2 off and give its run back to nobody: from its next latch
/// it fetches nothing.
pub fn hide(m: &mut dyn Mmio, p: Pipe) {
    let surf = m.read(r::plane_surf(p, PLANE));
    m.write(r::plane_ctl(p, PLANE), 0);
    m.write(r::plane_surf(p, PLANE), surf);
}
