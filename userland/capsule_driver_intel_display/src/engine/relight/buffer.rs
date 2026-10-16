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

//! A lit pipe's share of the display buffer and its plane's watermarks,
//! for a new line after a mode set. Alone on the engine the pipe gets the
//! whole buffer (`ddb::allocate`). Beside another lit pipe, or another
//! output held dark (`held`: its monitor pulled, its runs kept for its
//! return), it keeps the runs it has: those cannot overlap the other's,
//! and moving the other would mean rewriting its watermarks to latch at
//! its own blank. The
//! levels follow the new width, line total and clock (`wm::levels`) from
//! the latencies pcode reported, and latch with the plane's next surface.

use super::super::access::Mmio;
use super::super::error::Error;
use super::super::pipe::Pipe;
use super::super::plane_read::ENABLE;
use super::super::plane_regs::plane_ctl;
use super::super::wm_write::{cur_buf_cfg, plane_buf_cfg};
use super::super::{ddb, wm, wm_write};
use nonos_edid::Detailed;

pub fn buffer(
    m: &mut dyn Mmio,
    p: Pipe,
    width: u32,
    timing: &Detailed,
    latency_us: &[u32; 8],
    held: bool,
) -> Result<(), Error> {
    let others = Pipe::ALL[..3].iter().filter(|&&q| q != p);
    let lit = others.into_iter().any(|&q| m.read(plane_ctl(q, 1)) & ENABLE != 0);
    let alone = !lit && !held;
    let runs = if alone {
        let mut lit = [false; 3];
        lit[p.index() as usize] = true;
        ddb::allocate(lit)[p.index() as usize]
    } else {
        (ddb::Run::of(m.read(plane_buf_cfg(p))), ddb::Run::of(m.read(cur_buf_cfg(p))))
    };
    if runs.0.is_empty() {
        // Firmware gave a lit pipe no buffer: none is ours to take.
        return Err(Error::Layout);
    }
    let levels = wm::levels(latency_us, width, timing.h_total(), timing.pixel_khz, runs.0.len());
    wm_write::write(m, p, runs, &levels);
    Ok(())
}
