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

//! SCANOUT on a taken-over output (Gen9 on, universal planes): the band
//! is copied from the primary into the driver's band image and shown on
//! plane 2; the client's buffer, lent to the device read only through its
//! IOMMU domain, is mapped at one of two GGTT slots and plane 1 flips to
//! it at the vertical blank. The buffer shown before is unmapped and given
//! back only once plane 1 shows the new one. A FLUSH of the primary flips
//! plane 1 back to the driver's own surface and lets go of the client's
//! buffer and the band plane only after that flip (`leave`). Every refusal
//! leaves what was shown on screen.

use nonos_gfx_serve::Shadow;
use nonos_gfx_wire::errno::{EINVAL, ENOMEM, ENOSYS, ENXIO, ETIMEDOUT};
use nonos_gfx_wire::{Errno, Rect, Scanout};

use super::frames::{Frames, Lent};
use super::head::Head;
use super::intel::{Intel, FLIP_US};
use super::plug::Plug;
use crate::engine::access::{Clock, Mmio, Until};
use crate::engine::band_plane::{self, BYTES, PAGES, STRIDE};
use crate::engine::flip::wait_live;
use crate::engine::flip_park::flip_parked;
use crate::engine::gen::PlaneKind;
use crate::engine::ggtt;
use crate::engine::ggtt_run::map_run;
use crate::engine::irq_regs::irq_regs;
use crate::engine::park::Park;
use crate::engine::plane_regs as r;
use crate::engine::vblank::wait_vblank;

/// GGTT entries a plane surface is aligned to (256 KiB).
const ALIGN: u32 = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shown {
    pub lent: Lent,
    /// The GGTT slot (0 or 1) it is mapped at, and its entries.
    pub slot: usize,
    pub pages: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Direct {
    /// The client buffer plane 1 scans out now.
    pub shown: Option<Shown>,
    /// The band image's graphics address once mapped.
    pub band: Option<u32>,
    /// Plane 2's display buffer run was carved from plane 1's.
    pub carved: bool,
}

/// Where in the output's window the band image and the two client slots
/// of `pages` entries go: above the cursor's entries, each aligned, below
/// the lowest of the driver's own surfaces. None when they do not fit.
pub fn layout(window: (u32, u32), lowest_surface: u32, pages: u32) -> Option<(u32, [u32; 2])> {
    let band = (window.0 + 4).next_multiple_of(ALIGN);
    let span = pages.next_multiple_of(ALIGN);
    let first = band.checked_add(ALIGN)?;
    let end = first.checked_add(span.checked_mul(2)?)?;
    (end <= lowest_surface && end <= window.1).then_some((band, [first, first + span]))
}

/// The band's rows of `shadow` into the band image; its first IOVA.
fn band_image<F: Frames>(frames: &mut F, shadow: &Shadow, b: Rect) -> Option<u64> {
    let row = b.width as usize * 4;
    let mut iova = None;
    for y in 0..b.height as usize {
        let from = (b.y as usize + y) * shadow.stride as usize + b.x as usize * 4;
        let src = shadow.bytes.get(from..from.checked_add(row)?)?;
        if row > STRIDE as usize || (y + 1) * STRIDE as usize > BYTES {
            return None;
        }
        iova = Some(frames.band_row(y, src)?);
    }
    iova
}

impl<M: Mmio, F: Frames, C: Clock, P: Park> Intel<M, F, C, P> {
    pub fn scan_client(&mut self, o: u16, s: &Scanout, shadow: &Shadow) -> Result<(), Errno> {
        let Intel { m, clock, gen, heads, park, blank, .. } = self;
        let h = heads.get_mut(o as usize).and_then(|h| h.as_mut()).ok_or(ENXIO)?;
        if h.plug != Plug::In {
            return Err(ENXIO);
        }
        let (kind, pipe) = (h.scan.firmware.kind, h.scan.pipe);
        if kind == PlaneKind::Primary {
            return Err(ENOSYS);
        }
        let stride_ok = s.stride.is_multiple_of(64) && s.stride <= kind.max_stride();
        if (s.width, s.height) != (h.scan.width, h.scan.height) || !stride_ok {
            return Err(EINVAL);
        }
        let pages = (s.stride as u64 * s.height as u64).div_ceil(4096);
        let pages = u32::try_from(pages).map_err(|_| EINVAL)?;
        let lowest = h.scan.surfaces[0].min(h.scan.surfaces[1]) / 4096;
        let (band_at, slots) = layout(h.window, lowest, pages).ok_or(ENOMEM)?;
        let (t, top) = (h.out.ggtt, h.window.1);
        // The band first: from the primary into the band image.
        let iova = band_image(&mut h.frames, shadow, s.band).ok_or(EINVAL)?;
        h.frames.fence();
        if h.direct.band.is_none() {
            map_run(m, t, band_at, iova, PAGES, top).map_err(|_| ENOMEM)?;
            h.direct.band = Some(band_at * 4096);
        }
        if !h.direct.carved {
            band_plane::carve_regs(m, pipe).map_err(|_| ENOSYS)?;
            h.direct.carved = true;
        }
        let band = h.direct.band.ok_or(ENOMEM)?;
        let at = (s.band.x, s.band.y);
        let size = (s.band.width, s.band.height);
        let until = Until::new(clock, FLIP_US);
        // The same buffer again: only the band changed.
        if h.direct.shown.is_some_and(|sh| sh.lent.handle == s.handle) {
            band_plane::show(m, kind, pipe, band, at, size).map_err(|_| EINVAL)?;
            return wait_vblank(m, pipe, &until).map_err(|_| ETIMEDOUT);
        }
        let lent = h.frames.lend(s.handle)?;
        if lent.len < pages as u64 * 4096 {
            h.frames.give_back(lent);
            return Err(EINVAL);
        }
        let slot = h.direct.shown.map_or(0, |sh| 1 - sh.slot);
        if map_run(m, t, slots[slot], lent.iova, pages, top).is_err() {
            h.frames.give_back(lent);
            return Err(ENOMEM);
        }
        // Plane 2 is armed before plane 1: a blank between the two shows
        // the band over the primary's own band, the same pixels.
        if band_plane::show(m, kind, pipe, band, at, size).is_err() {
            unlend(m, h, slots[slot], pages, lent);
            return Err(EINVAL);
        }
        let was = m.read(r::plane_surflive(pipe, 1)) & !0xFFF;
        let was_stride = m.read(r::plane_stride(pipe, 1));
        m.write(r::plane_stride(pipe, 1), kind.stride_reg(s.stride));
        let irq = (irq_regs(gen, pipe), park as &mut dyn Park, &mut blank[pipe.index() as usize]);
        if flip_parked(m, pipe, slots[slot] * 4096, &until, irq).is_err() {
            // Put plane 1 back on what it showed before giving anything up.
            m.write(r::plane_stride(pipe, 1), was_stride);
            m.write(r::plane_surf(pipe, 1), was);
            let back = Until::new(clock, FLIP_US);
            if wait_live(m, pipe, was, &back).is_ok() {
                if h.direct.shown.is_none() {
                    band_plane::hide(m, pipe);
                }
                unlend(m, h, slots[slot], pages, lent);
            }
            return Err(ETIMEDOUT);
        }
        if let Some(old) = h.direct.shown.take() {
            unlend(m, h, slots[old.slot], old.pages, old.lent);
        }
        h.direct.shown = Some(Shown { lent, slot, pages });
        Ok(())
    }
}

/// Plane 1 shows the driver's own surface again (its stride set back
/// before the flip, `own_stride`): hide the band plane and give the client
/// buffer back.
pub fn leave<F: Frames>(m: &mut dyn Mmio, h: &mut Head<F>) {
    let Some(sh) = h.direct.shown.take() else { return };
    band_plane::hide(m, h.scan.pipe);
    let lowest = h.scan.surfaces[0].min(h.scan.surfaces[1]) / 4096;
    match layout(h.window, lowest, sh.pages) {
        Some((_, slots)) => unlend(m, h, slots[sh.slot], sh.pages, sh.lent),
        None => h.frames.give_back(sh.lent),
    }
}

/// Before a flip back to the driver's own surface: plane 1's stride as
/// that surface's, armed by the flip's PLANE_SURF.
pub fn own_stride<F: Frames>(m: &mut dyn Mmio, h: &Head<F>) {
    if h.direct.shown.is_some() {
        let kind = h.scan.firmware.kind;
        m.write(r::plane_stride(h.scan.pipe, 1), kind.stride_reg(h.scan.stride));
    }
}

fn unlend<F: Frames>(m: &mut dyn Mmio, h: &mut Head<F>, first: u32, pages: u32, lent: Lent) {
    let _ = ggtt::unmap(m, h.out.ggtt, first, pages, h.window.1);
    h.frames.give_back(lent);
}
