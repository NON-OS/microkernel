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

//! The adapter as a test holds it: BAR2 as a memory window (the EDID blob
//! at 0, the DISPI block at 0x500, as QEMU lays it out; unverified) and
//! video memory as a host buffer.

use nonos_devmodel::FakeBar;

use super::fence::sfence;
use crate::dispi::DispiMode;
use crate::display::Bga;
use crate::edid::Panel;
use crate::regs::Regs;

/// 16 MiB, QEMU's default for both stdvga and bochs-display (unverified).
pub const VRAM: usize = 16 << 20;
const BAR2: usize = 0x600;

pub struct Adapter {
    pub bar: FakeBar,
    pub vram: Vec<u8>,
}

impl Adapter {
    pub fn new() -> Self {
        Adapter { bar: FakeBar::new(BAR2), vram: vec![0; VRAM] }
    }

    pub fn regs(&self) -> Regs {
        Regs::new(self.bar.base())
    }

    /// Put an EDID blob at BAR2+0, as the device model presents it.
    pub fn edid(&self, blob: &[u8]) {
        for (i, b) in blob.iter().enumerate() {
            self.bar.present8(i, *b);
        }
    }

    pub fn bga(&mut self, panel: Panel, mode: DispiMode) -> Bga {
        let (vram, vram_len) = (self.vram.as_mut_ptr() as u64, self.vram.len());
        Bga { regs: self.regs(), vram, vram_len, panel, mode, fence: sfence }
    }
}
