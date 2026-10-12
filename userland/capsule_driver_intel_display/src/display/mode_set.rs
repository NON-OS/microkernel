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

//! MODE_SET on a taken-over output: the new surfaces mapped and filled
//! from the new primary before anything goes dark, then the output lit on
//! them. A failure puts the old mode back on the untouched old surfaces.

use nonos_gfx_serve::Shadow;
use nonos_gfx_wire::errno::{EINVAL, ENOMEM, ENOSYS, ETIMEDOUT};
use nonos_gfx_wire::{Errno, Rect};

use super::copy::copy;
use super::frames::Frames;
use super::intel::Intel;
use super::modeset::Modeset;
use super::timing_pick::pick;
use crate::engine::access::{Clock, Mmio};
use crate::engine::cdclk;
use crate::engine::error::Error;

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    pub fn change_mode(&mut self, w: u32, h: u32, shadow: &Shadow) -> Result<(), Errno> {
        let ms = self.modeset.ok_or(ENOSYS)?;
        let new = pick(&ms.monitor.blob[..ms.monitor.len], w, h);
        let new = new.filter(|t| cdclk::fits(&mut self.m, t.pixel_khz)).ok_or(EINVAL)?;
        let stride = (w * 4).next_multiple_of(64);
        let len = (stride * h).div_ceil(4096) as usize * 4096;
        let bases = self.frames.renew(len).ok_or(ENOMEM)?;
        let all = [Rect { x: 0, y: 0, width: w, height: h }];
        let filled =
            (0..2).try_for_each(|f| copy(&mut self.frames, f, (w, h), stride, &all, shadow));
        if let Err(e) = filled {
            self.frames.settle(false);
            return Err(e);
        }
        self.frames.fence();
        self.generation = self.generation.wrapping_add(1);
        let (was, old_bases) = ((self.scan.width, self.scan.height), self.bases);
        match self.light(&ms, &new, (w, h), bases) {
            Ok(()) => {
                self.frames.settle(true);
                self.bases = bases;
                self.modeset = Some(Modeset { now: new, ..ms });
                Ok(())
            }
            Err(e) => {
                self.frames.settle(false);
                let _ = self.light(&ms, &ms.now, was, old_bases);
                Err(errno(e))
            }
        }
    }
}

fn errno(e: Error) -> Errno {
    match e {
        Error::Timeout => ETIMEDOUT,
        Error::GgttFull => ENOMEM,
        _ => EINVAL,
    }
}
