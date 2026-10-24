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
//! A surface: fresh DMA (zeroed by the kernel when mapped), a 2D resource
//! of its size on the host, and the one backing entry joining them. Made
//! whole or not at all; given back as `limbo.rs` says.

use nonos_gfx_wire::errno::{ENOMEM, ETIMEDOUT};
use nonos_gfx_wire::Errno;
use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut, MK_DMA_MAP_HIGH};

use super::gpu::Gpu;
use crate::constants::VG_FORMAT_B8G8R8A8_UNORM;
use crate::device::cmd::{attach_backing, create_resource_2d};

pub struct Surface {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub map: DmaMapOut,
    pub attached: bool,
}

impl Surface {
    pub fn bytes(&self) -> usize {
        self.width as usize * self.height as usize * 4
    }
}

impl Gpu {
    pub fn make(&mut self, width: u32, height: u32) -> Result<Surface, Errno> {
        let len = width as u64 * height as u64 * 4;
        let mut map = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
        let (dev, epoch) = self.device;
        let mapped = mk_dma_map(dev, epoch, len.next_multiple_of(4096), MK_DMA_MAP_HIGH, &mut map);
        if mapped < 0 || map.user_va == 0 || map.device_addr == 0 {
            return Err(ENOMEM);
        }
        let (id, fmt) = (self.fresh_id(), VG_FORMAT_B8G8R8A8_UNORM);
        let (q, f) = (&self.q, &self.fences);
        let made = create_resource_2d(q, f.issue(), id, fmt, width, height).is_ok();
        let attached =
            made && attach_backing(q, f.issue(), id, map.device_addr, len as u32).is_ok();
        if attached {
            return Ok(Surface { id, width, height, map, attached });
        }
        // An attach that timed out may have landed: given back as attached.
        match made {
            true => self.give_back(Surface { id, width, height, map, attached: true }),
            false => {
                let _ = mk_dma_unmap(map.grant_id);
            }
        }
        Err(ETIMEDOUT)
    }
}
