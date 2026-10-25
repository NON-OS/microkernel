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
//! A mode set makes before it breaks: a new surface of the size, the
//! shadow loaded into it whole, and only then the scanout pointed at it,
//! so a step that fails leaves the old surface showing and gives the new
//! one back. Once the scanout shows the new surface the old one is given
//! back (to limbo, if the host keeps it). A lost flush after the switch
//! is not undone: the picture is on the host and the next flush shows it.

use nonos_gfx_serve::Shadow;
use nonos_gfx_wire::errno::{EAGAIN, EINVAL, ETIMEDOUT};
use nonos_gfx_wire::{Errno, Rect};

use super::gpu::Gpu;
use super::plug::Plug;
use super::show::area;
use crate::device::cmd::set_scanout;

impl Gpu {
    pub fn mode_set(&mut self, output: u16, w: u32, h: u32, shadow: &Shadow) -> Result<(), Errno> {
        self.retry_limbo();
        let head = self.head(output).ok_or(EINVAL)?;
        // Off: there is no screen to set a mode on until the host is back.
        if head.plug == Plug::Out {
            return Err(EAGAIN);
        }
        let scanout = head.scanout;
        let new = self.make(w, h)?;
        let whole = [Rect { x: 0, y: 0, width: w, height: h }];
        let ready = self.load(&new, &whole, shadow);
        let on = ready.and_then(|_| {
            set_scanout(&self.q, self.fences.issue(), scanout, new.id, area(&whole[0]))
                .map_err(|_| ETIMEDOUT)
        });
        if let Err(e) = on {
            self.give_back(new);
            return Err(e);
        }
        let _ = self.flush_rects(&new, &whole);
        let head = self.heads[output as usize].as_mut().ok_or(EINVAL)?;
        head.plug = Plug::In;
        let old = core::mem::replace(&mut head.shown, new);
        self.give_back(old);
        Ok(())
    }
}
