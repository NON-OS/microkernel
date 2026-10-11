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

//! The data bus width, and the widths and timings the host may run.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::regs::{HC_4BIT, HC_8BIT, HOST_CONTROL};
use super::engine::Host;

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    /// Data Transfer Width and Extended Data Transfer Width for 1, 4 or 8
    /// lines.
    pub fn set_width(&mut self, bits: u8) {
        let mut hc = self.io.r8(HOST_CONTROL) & !(HC_4BIT | HC_8BIT);
        match bits {
            8 => hc |= HC_8BIT,
            4 => hc |= HC_4BIT,
            _ => {}
        }
        self.io.w8(HOST_CONTROL, hc);
        self.width = if bits == 8 || bits == 4 { bits } else { 1 };
    }

    /// The host may drive eight data lines: the capabilities say so, or it
    /// is a known Intel eMMC host.
    pub fn can_bus8(&self) -> bool {
        self.caps.bus8() || self.intel_emmc
    }

    /// The host may run High Speed timing.
    pub fn can_hs(&self) -> bool {
        self.caps.high_speed() || self.intel_emmc
    }
}
