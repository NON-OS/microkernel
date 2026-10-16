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

//! MODE_SET on a taken-over output: the new surfaces mapped and filled from
//! the new primary before anything goes dark, then the output lit on them; a
//! failure puts the old mode back on the untouched old surfaces. A pulled
//! output is refused (EAGAIN); one lit, or back on its old mode, is In.

use nonos_gfx_serve::Shadow;
use nonos_gfx_wire::errno::{EAGAIN, EINVAL, ENOMEM, ENOSYS, ENXIO};
use nonos_gfx_wire::{Errno, Rect};

use super::copy::copy;
use super::frames::Frames;
use super::intel::Intel;
use super::mode_light::{errno, light};
use super::modeset::Modeset;
use super::plug::Plug;
use super::timing_pick::pick;
use crate::engine::access::{Clock, Mmio};
use crate::engine::cdclk;

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    pub fn change_mode(&mut self, o: u16, w: u32, h: u32, shadow: &Shadow) -> Result<(), Errno> {
        let held = self.held_beside(o);
        let Intel { m, clock, heads, .. } = self;
        let hd = heads.get_mut(o as usize).and_then(|h| h.as_mut()).ok_or(ENXIO)?;
        let ms = hd.modeset.ok_or(ENOSYS)?;
        if hd.plug == Plug::Out {
            return Err(EAGAIN);
        }
        let new = pick(&ms.monitor.blob[..ms.monitor.len], w, h);
        let new = new.filter(|t| cdclk::fits(m, t.pixel_khz)).ok_or(EINVAL)?;
        let stride = (w * 4).next_multiple_of(64);
        let len = (stride * h).div_ceil(4096) as usize * 4096;
        let bases = hd.frames.renew(len).ok_or(ENOMEM)?;
        let all = [Rect { x: 0, y: 0, width: w, height: h }];
        let filled = (0..2).try_for_each(|f| copy(&mut hd.frames, f, (w, h), stride, &all, shadow));
        if let Err(e) = filled {
            hd.frames.settle(false);
            return Err(e);
        }
        hd.frames.fence();
        hd.generation = hd.generation.wrapping_add(1);
        let (was, old_bases) = ((hd.scan.width, hd.scan.height), hd.bases);
        match light(m, clock, &mut hd.scan, (&ms, &new), (w, h), bases, held) {
            Ok(()) => {
                hd.frames.settle(true);
                hd.bases = bases;
                (hd.modeset, hd.plug) = (Some(Modeset { now: new, ..ms }), Plug::In);
                Ok(())
            }
            Err(e) => {
                hd.frames.settle(false);
                if light(m, clock, &mut hd.scan, (&ms, &ms.now), was, old_bases, held).is_ok() {
                    hd.plug = Plug::In;
                }
                Err(errno(e))
            }
        }
    }
}
