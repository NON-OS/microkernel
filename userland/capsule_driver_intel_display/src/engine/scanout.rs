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

//! A taken-over output: two surfaces of the firmware's size in the GGTT,
//! one scanned out while the other is drawn, swapped by a flip at the
//! vertical blank. The mode, the pipe and the port stay as firmware set
//! them; only the plane's surface changes.

use super::access::{Expiry, Mmio};
use super::error::Error;
use super::flip::flip;
use super::ggtt_pte::Pte;
use super::pipe::Pipe;
use super::plane_read::PlaneState;

pub struct Scanout {
    pub pipe: Pipe,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    /// Each surface's graphics address and its run of GGTT entries.
    pub surfaces: [u32; 2],
    pub pages: u32,
    pub ggtt: Pte,
    pub front: usize,
    /// The plane as firmware left it, put back by `stop`.
    pub firmware: PlaneState,
}

impl Scanout {
    /// The surface to draw the next frame into.
    pub fn back(&self) -> usize {
        1 - self.front
    }

    /// Show the back surface from the next blank; returns once it is
    /// live, so the old front may be drawn into at once.
    pub fn present(&mut self, m: &mut dyn Mmio, until: &dyn Expiry) -> Result<(), Error> {
        flip(m, self.pipe, self.surfaces[self.back()], until)?;
        self.front = self.back();
        Ok(())
    }
}

/// Whether lit surface `p` reaches into the GGTT entries from `ours` up,
/// the ones this driver's windows take: a dead driver's leftover, not
/// firmware's picture (G-72). Never copied; mapped over, not avoided.
pub fn leftover(p: &PlaneState, ours: u32) -> bool {
    let (first, pages) = surface_run(p);
    first as u64 + pages as u64 > ours as u64
}

/// The GGTT entries lit surface `p` takes: (first, count).
pub fn surface_run(p: &PlaneState) -> (u32, u32) {
    (p.live / 4096, (p.stride as u64 * p.height as u64).div_ceil(4096) as u32)
}
