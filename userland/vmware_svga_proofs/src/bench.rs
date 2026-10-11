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

//! An adapter on host memory: its FIFO and VRAM as the BARs the broker
//! maps, and the device record discovery would hand the driver.

use nonos_libc::set_bar;

use crate::constants::pci::{Layout, VMWARE_LAYOUT};
use crate::discover::Found;
use crate::model::{install, Svga};

pub struct Bench {
    pub fifo: Vec<u32>,
    pub vram: Vec<u8>,
}

impl Bench {
    pub fn new(fifo_bytes: usize, vram_bytes: usize) -> Self {
        Self { fifo: vec![0; fifo_bytes / 4], vram: vec![0; vram_bytes] }
    }

    /// The adapter installed, tuned by `tune`, and what discovery found.
    pub fn with(&mut self, tune: impl FnOnce(&mut Svga)) -> Found {
        self.laid_out(VMWARE_LAYOUT, tune)
    }

    /// The same with its BARs in `layout`'s order.
    pub fn laid_out(&mut self, layout: Layout, tune: impl FnOnce(&mut Svga)) -> Found {
        let fifo = (self.fifo.as_mut_ptr() as u64, self.fifo.len() * 4);
        let vram = (self.vram.as_mut_ptr() as u64, self.vram.len() as u64);
        set_bar(layout.fb as usize, vram.0, vram.1);
        set_bar(layout.fifo as usize, fifo.0, fifo.1 as u64);
        let mut svga = Svga::new(fifo);
        svga.vram = vram.1 as u32;
        tune(&mut svga);
        install(svga);
        Found { device_id: 1, layout, fb_bytes: vram.1, fifo_bytes: fifo.1 as u64 }
    }

    pub fn standard() -> Self {
        Self::new(64 << 10, 16 << 20)
    }
}
