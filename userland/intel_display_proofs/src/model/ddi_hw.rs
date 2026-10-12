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

//! A DDI in the model, wired to the sink behind its AUX channel: the
//! buffer's width and translation entry and the routed DPLL's rate become
//! the link the sink sees, the transport's pattern the pattern it gets.
//! An enabled DPLL locks a few reads later; a disabled buffer goes idle.

use super::gen9::Gen9;
use crate::dp::lane::Drive;
use crate::dp::train::Pattern;
use crate::engine::ddi::*;
use crate::engine::dpll::{enable_reg, DPLL_CTRL1, DPLL_CTRL2, DPLL_STATUS};

const RATES: [u32; 6] = [5400, 2700, 1620, 3240, 2160, 4320];
const DRIVES: [(u8, u8); 10] =
    [(0, 0), (0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (3, 0)];

impl Gen9 {
    pub(super) fn ddi_write(&mut self, off: u32, v: u32) {
        for n in 1..=3 {
            if off == enable_reg(n) && v & (1 << 31) != 0 && !self.dead {
                let st = self.get(DPLL_STATUS) | (1 << (n * 8));
                self.pending.push((2, DPLL_STATUS, st));
            }
        }
        let Some((ddi, sink)) = self.sink.clone() else { return };
        let d = ddi as u32;
        let mut s = sink.borrow_mut();
        if off == tp_ctl(d) {
            s.pattern = match (v & TP_ENABLE != 0, v & PATTERN_MASK) {
                (true, TPS1) => Pattern::Tps1,
                (true, TPS2) => Pattern::Tps2,
                (true, TPS3) => Pattern::Tps3,
                _ => Pattern::Off,
            };
        }
        if off == buf_ctl(d) {
            if v & BUF_ENABLE == 0 {
                s.link = None;
                self.regs.insert(off, v | BUF_IDLE);
                return;
            }
            let dpll = (self.get(DPLL_CTRL2) >> (d * 3 + 1)) & 3;
            let code = (self.get(DPLL_CTRL1) >> (dpll * 6 + 1)) & 7;
            let lanes = [1, 2, 0, 4][((v & WIDTH_MASK) >> 1) as usize & 3];
            s.link = Some((RATES[code as usize % 6], lanes));
            let (swing, pre) = DRIVES[((v & TRANS_MASK) >> TRANS_SHIFT) as usize % 10];
            s.drive = Drive { swing, pre };
        }
    }
}
