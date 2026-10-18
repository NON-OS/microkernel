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
//! The heads shown again after a recovery reset the device:
//! a completed reset has the host drop every resource and backing, so each
//! head's surface is made again under its own id on its own pages, which
//! the driver kept mapped, loaded whole from them and, for a head the host
//! has on, set on its scanout and flushed. Nothing is drawn anew: the
//! pages hold the last frame the compositor sent. A surface in limbo was
//! waiting for the host to let go of it, and a reset host holds none, so
//! its pages go back to the kernel now.

use nonos_gfx_wire::errno::ETIMEDOUT;
use nonos_gfx_wire::{Errno, Rect};
use nonos_libc::mk_dma_unmap;

use super::gpu::Gpu;
use super::plug::Plug;
use super::show::area;
use crate::constants::VG_FORMAT_B8G8R8A8_UNORM;
use crate::device::cmd::{attach_backing, create_resource_2d, set_scanout};

impl Gpu {
    pub fn reshow(&mut self) -> Result<(), Errno> {
        for s in core::mem::take(&mut self.limbo) {
            let _ = mk_dma_unmap(s.map.grant_id);
        }
        for i in 0..self.heads.len() {
            let Some(h) = self.heads[i].as_ref() else { continue };
            let (s, scanout, on) = (&h.shown, h.scanout, h.plug == Plug::In);
            let (q, f) = (&self.q, &self.fences);
            let fmt = VG_FORMAT_B8G8R8A8_UNORM;
            create_resource_2d(q, f.issue(), s.id, fmt, s.width, s.height)
                .and_then(|_| {
                    attach_backing(q, f.issue(), s.id, s.map.device_addr, s.bytes() as u32)
                })
                .map_err(|_| ETIMEDOUT)?;
            if !on {
                continue;
            }
            let whole = [Rect { x: 0, y: 0, width: s.width, height: s.height }];
            self.send(s, &whole)?;
            set_scanout(q, f.issue(), scanout, s.id, area(&whole[0])).map_err(|_| ETIMEDOUT)?;
            self.flush_rects(s, &whole)?;
        }
        Ok(())
    }
}
