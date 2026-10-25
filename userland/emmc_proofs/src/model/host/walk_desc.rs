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

//! Where the ADMA2 table starts and how wide its descriptors are, and one
//! descriptor read from host memory.

use super::sim::Sim;
use super::types::Desc;

impl Sim {
    /// The table address the registers name and whether its descriptors are
    /// 64-bit; None when the DMA select names no ADMA2.
    pub(super) fn table_start(&mut self) -> Option<(u64, bool)> {
        let sel = (self.regs[0x28] >> 3) & 3;
        let wide = match sel {
            2 => false,
            3 => true,
            _ => {
                self.violate(format!("data command with DMA select {sel}"));
                return None;
            }
        };
        if wide && self.cfg.caps & 1 << 28 == 0 {
            self.violate("64-bit ADMA2 on a host without it".into());
        }
        let mut at = self.get(0x58, 4) as u64;
        if wide {
            at |= (self.get(0x5c, 4) as u64) << 32;
        }
        if !at.is_multiple_of(if wide { 8 } else { 4 }) {
            self.violate(format!("descriptor table at {at:#x} misaligned"));
        }
        Some((at, wide))
    }

    pub(super) fn desc(&mut self, at: u64, wide: bool) -> Desc {
        let dl = if wide { 12 } else { 8 };
        let p = self.ptr(at, dl);
        let d = unsafe { std::slice::from_raw_parts(p, dl) };
        let attr = u16::from_le_bytes([d[0], d[1]]);
        let l = u16::from_le_bytes([d[2], d[3]]) as u32;
        let addr = if wide {
            u64::from_le_bytes(d[4..12].try_into().unwrap())
        } else {
            u32::from_le_bytes(d[4..8].try_into().unwrap()) as u64
        };
        Desc { attr, len: l, addr }
    }
}
