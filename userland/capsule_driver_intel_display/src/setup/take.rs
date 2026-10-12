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

//! Taking the lit output over: two DMA surfaces holding firmware's picture,
//! the plane moved onto the first, the monitor's EDID read, mode setting
//! found where it can be had, and the wire server made over all of it.

use alloc::vec::Vec;

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;
use nonos_gfx_wire::body::state::SCANNING_OUT;
use nonos_intel_display::display::{Frames, Intel, Modeset, Monitor, FLIP_US};
use nonos_intel_display::engine::access::Until;
use nonos_intel_display::engine::gen::Gen;
use nonos_intel_display::engine::{scanout::Scanout, takeover::Output};

use super::{firmware, Driver};
use crate::bar::Bar0;
use crate::clock::Uptime;
use crate::dma_pair::map_pair;
use crate::error::{IntelError, IntelResult};
use crate::frames::DmaFrames;
use crate::handles::BrokerHandles;

pub fn take(
    h: BrokerHandles,
    mut m: Bar0,
    gen: &'static Gen,
    out: Output,
    total: u32,
    aperture: u64,
) -> IntelResult<Driver> {
    let clock = Uptime;
    let stride = (out.width * 4).next_multiple_of(64);
    let pages = (stride * out.height).div_ceil(4096);
    let now = map_pair(h.device_id, h.epoch, pages as u64 * 4096)?;
    let mut frames = DmaFrames { device_id: h.device_id, epoch: h.epoch, now, was: None };
    firmware::copy_picture(&h, &out.plane, &mut frames, stride, aperture);
    frames.fence();
    let iovas = |base: u64| (0..pages as u64).map(|i| base + i * 4096).collect::<Vec<u64>>();
    let (a, b) = (iovas(now.iova[0]), iovas(now.iova[1]));
    let until = Until::new(&clock, FLIP_US);
    let scan = Scanout::start(&mut m, &out, total, [&a, &b], &until).map_err(IntelError::Engine)?;
    let monitor = Monitor::of(&mut m, &clock, gen, out.transcoder);
    let modeset = Modeset::find(&mut m, &clock, gen, &out, &monitor, total);
    let (panel, bases, flags) = (monitor.panel(), now.iova, SCANNING_OUT);
    let intel =
        Intel { m, frames, clock, gen, out, scan, panel, modeset, bases, generation: 1, flags };
    Ok(Driver { server: Server::new(intel, Pool), _handles: h })
}
