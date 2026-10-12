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

//! One bring-up attempt: claim, BAR0, the GGTT's size, DC states off, the
//! first output firmware lit, two DMA scanout surfaces holding firmware's
//! picture, and the plane moved onto the first. Every engine step checks
//! before it writes, and every grant is given back by the handles' drop
//! when a step fails, so the next attempt starts clean.

use alloc::vec::Vec;

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;
use nonos_gfx_wire::body::state::SCANNING_OUT;
use nonos_intel_display::display::{Frames, Intel, FLIP_US};
use nonos_intel_display::engine::access::Until;
use nonos_intel_display::engine::error::Error;
use nonos_intel_display::engine::{dc::dc_off, gen::PlaneKind, ggtt_range::entries};
use nonos_intel_display::engine::{scanout::Scanout, takeover::read_outputs};

use super::{claim, firmware, maps, Driver};
use crate::bar::Bar0;
use crate::clock::Uptime;
use crate::discover::Found;
use crate::error::{IntelError, IntelResult};
use crate::frames::DmaFrames;

/// DC_STATE_EN reads back off within a few milliseconds (unverified).
const DC_US: u64 = 50_000;

pub fn run(dev: Found) -> IntelResult<Driver> {
    let mut h = claim::claim(dev.device_id)?;
    let gen = dev.family.gen();
    let total = entries(gen.ggtt, claim::ggc(&h)?);
    if total == 0 {
        return Err(IntelError::NoGgtt);
    }
    let map = maps::bar0(&mut h, dev.bar0_size)?;
    let (mut m, clock) = (Bar0 { base: map.user_va, len: map.length }, Uptime);
    // Gen8 and Haswell have no DC states (`dc`).
    if gen.plane != PlaneKind::Primary {
        dc_off(&mut m, &Until::new(&clock, DC_US)).map_err(IntelError::Engine)?;
    }
    let lit = read_outputs(&mut m, gen).into_iter().flatten().next();
    let out = lit.ok_or(IntelError::Engine(Error::NothingLit))?;
    let stride = (out.width * 4).next_multiple_of(64);
    let pages = (stride * out.height).div_ceil(4096);
    let len = pages as u64 * 4096;
    let (f0, f1) = (maps::frame(&mut h, 0, len)?, maps::frame(&mut h, 1, len)?);
    let mut frames = DmaFrames { va: [f0.user_va, f1.user_va], len: len as usize };
    firmware::copy_picture(&h, &out.plane, &mut frames, stride, dev.bar2_size);
    frames.fence();
    let iovas = |base: u64| (0..pages as u64).map(|i| base + i * 4096).collect::<Vec<u64>>();
    let (a, b) = (iovas(f0.device_addr), iovas(f1.device_addr));
    let until = Until::new(&clock, FLIP_US);
    let scan = Scanout::start(&mut m, &out, total, [&a, &b], &until).map_err(IntelError::Engine)?;
    let flags = SCANNING_OUT;
    let intel = Intel { m, frames, clock, gen, out, scan, generation: 1, flags };
    Ok(Driver { _handles: h, server: Server::new(intel, Pool) })
}
