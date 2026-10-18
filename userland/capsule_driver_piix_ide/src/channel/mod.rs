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

//! One IDE channel: its command and control blocks and its half of the bus
//! master window. Two drives share it, and only the selected one answers.

mod probe;
mod reset;
mod select;

pub use probe::Probe;

use crate::constants::ata::{REG_STATUS, ST_BSY};
use crate::constants::bmide::channel_base;
use crate::constants::pci::control_offset;
use crate::regs::Port;

#[derive(Clone, Copy)]
pub struct Channel {
    pub cmd: Port,
    ctl: Port,
    ctl_off: u16,
    bm: Port,
    bm_base: u16,
}

impl Channel {
    /// `ctl_ports` is the control window's size, which says where in it the
    /// register sits (`control_offset`).
    pub fn new(index: usize, cmd: Port, ctl: Port, ctl_ports: u16, bm: Port) -> Self {
        Self { cmd, ctl, ctl_off: control_offset(ctl_ports), bm, bm_base: channel_base(index) }
    }

    /// The status without clearing a pending interrupt.
    pub fn alt_status(&self) -> u8 {
        self.ctl.read8(self.ctl_off)
    }

    /// The status, which also clears the device's pending interrupt.
    pub fn status(&self) -> u8 {
        self.cmd.read8(REG_STATUS)
    }

    pub fn control(&self, v: u8) {
        self.ctl.write8(self.ctl_off, v);
    }

    pub fn bm_read8(&self, reg: u16) -> u8 {
        self.bm.read8(self.bm_base + reg)
    }

    pub fn bm_write8(&self, reg: u16, v: u8) {
        self.bm.write8(self.bm_base + reg, v);
    }

    pub fn bm_write32(&self, reg: u16, v: u32) {
        self.bm.write32(self.bm_base + reg, v);
    }

    /// Whether the selected device has let go of the bus within `ms`.
    pub fn not_busy(&self, ms: u64) -> bool {
        crate::wait::until(ms, || self.alt_status() & ST_BSY == 0)
    }
}
