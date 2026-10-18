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
//! The device as the wire server drives it: the control queue, the heads
//! (one per output, each naming its scanout and the surface on it), the
//! monitor's EDID for output 0, and surfaces the host did not let go of.

use alloc::vec::Vec;
use nonos_gfx_serve::{Panel, MAX_OUTPUTS};

use super::surface::Surface;
use crate::device::ControlQueue;
use crate::state::{FenceCounter, Health};

/// The largest side offered; a host takes any size, DMA bounds it here.
pub const MAX_SIDE: u32 = 4096;
/// One display DMA grant: the broker's DISPLAY_FRAMEBUFFER_PAGES, 8192 of
/// 4 KiB. A surface is one grant, so no mode over it is offered.
pub const GRANT_BYTES: u64 = 8192 * 4096;

pub struct Head {
    pub scanout: u32,
    pub shown: Surface,
    /// The size the host's display info asked for this scanout.
    pub hint: (u32, u32),
    pub plug: super::plug::Plug,
}

pub struct Gpu {
    pub q: ControlQueue,
    pub fences: FenceCounter,
    /// The device and claim its DMA is mapped for.
    pub device: (u64, u64),
    pub heads: [Option<Head>; MAX_OUTPUTS],
    pub outputs: u16,
    pub panel: Panel,
    pub generation: u32,
    pub next_id: u32,
    /// Surfaces whose backing the host kept: their pages stay mapped, never
    /// handed back to the kernel while the host may still read them.
    pub limbo: Vec<Surface>,
    /// Down once a stalled queue could not be brought back (`recover`).
    pub health: Health,
}

impl Gpu {
    pub fn head(&self, output: u16) -> Option<&Head> {
        self.heads.get(output as usize)?.as_ref()
    }

    /// An id no surface of this driver's holds. A reset before bring-up
    /// left the host none, so ids are this driver's alone.
    pub fn fresh_id(&mut self) -> u32 {
        loop {
            let id = self.next_id;
            self.next_id = id.wrapping_add(1).max(1);
            let shown = self.heads.iter().flatten().any(|h| h.shown.id == id);
            if !shown && !self.limbo.iter().any(|s| s.id == id) {
                return id;
            }
        }
    }
}
