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

//! Writing a pipe's buffer runs and watermarks (SKL PRM Vol 2c,
//! PLANE_BUF_CFG, PLANE_WM, PLANE_WM_TRANS, CUR_BUF_CFG, CUR_WM; offsets
//! as the instance lists give them, 0x100 per plane, 0x1000 per pipe).
//! They are double-buffered with the plane and latch with it.

use super::access::Mmio;
use super::ddb::Run;
use super::pipe::Pipe;
use super::wm::Level;

pub fn plane_buf_cfg(p: Pipe) -> u32 {
    0x7_027C + 0x1000 * p.index()
}
pub fn plane_wm(p: Pipe, level: u32) -> u32 {
    0x7_0240 + 0x1000 * p.index() + 4 * level
}
pub fn plane_wm_trans(p: Pipe) -> u32 {
    0x7_0268 + 0x1000 * p.index()
}
pub fn cur_buf_cfg(p: Pipe) -> u32 {
    0x7_017C + 0x1000 * p.index()
}
pub fn cur_wm(p: Pipe, level: u32) -> u32 {
    0x7_0140 + 0x1000 * p.index() + 4 * level
}

/// The primary plane's run and levels, the cursor's run with its level 0
/// at the smallest the PRM allows (its 64 pixel line fits one block),
/// transition watermarks off.
pub fn write(m: &mut dyn Mmio, p: Pipe, runs: (Run, Run), levels: &[Level; 8]) {
    m.write(plane_buf_cfg(p), runs.0.cfg());
    for (i, l) in levels.iter().enumerate() {
        m.write(plane_wm(p, i as u32), l.reg());
    }
    m.write(plane_wm_trans(p), 0);
    m.write(cur_buf_cfg(p), runs.1.cfg());
    let cursor0 = Level { enabled: !runs.1.is_empty(), lines: 1, blocks: runs.1.len().min(8) };
    m.write(cur_wm(p, 0), cursor0.reg());
    for i in 1..8 {
        m.write(cur_wm(p, i), 0);
    }
}
