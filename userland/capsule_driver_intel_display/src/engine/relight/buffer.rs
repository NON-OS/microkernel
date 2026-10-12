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
//! for a new line after a mode set: the pipe alone on the engine gets the
//! whole buffer (`ddb::allocate`), and the levels follow the new width,
//! line total and clock (`wm::levels`) from the latencies pcode reported.
//! Double-buffered with the plane, they latch with its next surface write.

use super::super::access::Mmio;
use super::super::pipe::Pipe;
use super::super::{ddb, wm, wm_write};
use nonos_edid::Detailed;

pub fn buffer(m: &mut dyn Mmio, p: Pipe, width: u32, timing: &Detailed, latency_us: &[u32; 8]) {
    let mut lit = [false; 3];
    lit[p.index() as usize] = true;
    let runs = ddb::allocate(lit)[p.index() as usize];
    let levels = wm::levels(latency_us, width, timing.h_total(), timing.pixel_khz, runs.0.len());
    wm_write::write(m, p, runs, &levels);
}
