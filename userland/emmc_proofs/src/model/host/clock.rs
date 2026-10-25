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

//! The bus width and SD clock the registers set.

use super::sim::Sim;

impl Sim {
    pub fn host_width(&self) -> u8 {
        let hc = self.regs[0x28];
        if hc & 0x20 != 0 {
            8
        } else if hc & 0x02 != 0 {
            4
        } else {
            1
        }
    }

    /// The SD clock in Hz, 0 when off.
    pub fn sd_hz(&self) -> u32 {
        let clk = self.get(0x2c, 2);
        if clk & 0x4 == 0 || clk & 0x1 == 0 {
            return 0;
        }
        let base = ((self.cfg.caps >> 8) & 0xff) * 1_000_000;
        let n = ((clk >> 8) & 0xff) | ((clk >> 6) & 0x3) << 8;
        if (self.cfg.version & 0xff) >= 2 {
            if n == 0 {
                base
            } else {
                base / (2 * n)
            }
        } else {
            let n = (clk >> 8) & 0xff;
            if n == 0 {
                base
            } else {
                base / (2 * n)
            }
        }
    }
}
