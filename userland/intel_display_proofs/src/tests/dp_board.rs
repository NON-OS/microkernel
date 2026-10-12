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

//! The board under the DP rig: firmware's timing on transcoder and pipe B
//! from DDI B (DisplayPort, four lanes) on DPLL 1, the monitor behind DDI
//! B's AUX, firmware's surface in the GGTT, and the two scanout surfaces'
//! pages filled 0xAA and 0xBB.

use crate::engine::aux_regs::Ddi;
use crate::engine::cdclk::CDCLK_CTL;
use crate::engine::ddi::buf_trans;
use crate::engine::dpll::DPLL_CTRL2;
use crate::engine::ggtt_pte::Pte;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::model::firmware::Axis;
use crate::model::sink::{sink, Sink};
use crate::model::Gen9;

pub fn dp_board(
    timing: (Axis, Axis),
    edid: Vec<u8>,
    total: u32,
) -> (Gen9, Sink, Vec<u64>, Vec<u64>) {
    let ((w, ..), (h, ..)) = timing;
    let mut m = Gen9::new(total as usize);
    m.light(Transcoder::B, Pipe::B, timing, (1 << 28) | (2 << 24) | (3 << 1));
    m.set(DPLL_CTRL2, (1 | (1 << 1)) << 3);
    let s = sink(edid);
    m.sink = Some((Ddi::B, s.clone()));
    m.set(buf_trans(1, 0), 0x0000_2016);
    m.set(CDCLK_CTL, 1349);
    let pages = ((w * 4).next_multiple_of(64) * h).div_ceil(4096) as u64;
    m.fill(0x9000_0000, 1, 0xFF00_0001);
    let fw: Vec<u64> = vec![0x9000_0000; pages as usize];
    crate::engine::ggtt::map(&mut m, Pte::Wide, 0, &fw, total).unwrap();
    m.fill(0x1000_0000, pages, 0xFF00_00AA);
    m.fill(0x2000_0000, pages, 0xFF00_00BB);
    let a: Vec<u64> = (0..pages).map(|i| 0x1000_0000 + i * 4096).collect();
    let b: Vec<u64> = (0..pages).map(|i| 0x2000_0000 + i * 4096).collect();
    (m, s, a, b)
}
