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

//! One output taken over: two DMA surfaces holding firmware's picture, the
//! plane moved onto the first inside the output's GGTT window, and mode
//! setting found for it where it can be had.

use alloc::vec::Vec;

use nonos_gfx_wire::body::state::SCANNING_OUT;
use nonos_intel_display::display::{Frames, Head, Modeset, Monitor, FLIP_US};
use nonos_intel_display::engine::access::{Clock, Until};
use nonos_intel_display::engine::gen::Gen;
use nonos_intel_display::engine::{scanout::Scanout, takeover::Output};

use super::firmware;
use crate::bar::Bar0;
use crate::dma_pair::map_pair;
use crate::error::{IntelError, IntelResult};
use crate::frames::DmaFrames;
use crate::handles::BrokerHandles;

pub fn take_head(
    h: &BrokerHandles,
    m: &mut Bar0,
    (clock, gen): (&dyn Clock, &Gen),
    out: Output,
    window: (u32, u32),
    monitor: &Monitor,
    aperture: u64,
) -> IntelResult<Head<DmaFrames>> {
    let stride = (out.width * 4).next_multiple_of(64);
    let pages = (stride * out.height).div_ceil(4096);
    let now = map_pair(h.device_id, h.epoch, pages as u64 * 4096)?;
    let mut frames = DmaFrames { device_id: h.device_id, epoch: h.epoch, now, was: None };
    firmware::copy_picture(h, &out.plane, &mut frames, stride, aperture);
    frames.fence();
    let iovas = |base: u64| (0..pages as u64).map(|i| base + i * 4096).collect::<Vec<u64>>();
    let (a, b) = (iovas(now.iova[0]), iovas(now.iova[1]));
    let until = Until::new(clock, FLIP_US);
    let scan = Scanout::start(m, &out, window.1, [&a, &b], &until).map_err(IntelError::Engine)?;
    let modeset = Modeset::find(m, clock, gen, &out, monitor, window);
    let (panel, bases, flags, generation) = (monitor.panel(), now.iova, SCANNING_OUT, 1);
    Ok(Head { out, scan, frames, panel, modeset, bases, generation, flags })
}

/// GGTT entries a surface of `w` x `h` takes, in 64-entry steps (the
/// 256 KiB alignment a linear surface needs).
pub fn run(w: u32, h: u32) -> u32 {
    ((w * 4).next_multiple_of(64) * h).div_ceil(4096).next_multiple_of(64)
}
