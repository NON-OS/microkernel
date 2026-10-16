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

//! Taking every lit output over, pipe A's first. Each gets its own window
//! of the GGTT, stacked down from the top, room for two surfaces of the
//! largest mode its monitor offers, and every window lies above all of
//! firmware's surfaces: no output's surfaces can land on another's, or on
//! a picture firmware is still scanning out. An output that does not fit,
//! or fails, is left as firmware lit it; none at all fails the attempt.

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;
use nonos_intel_display::display::{Intel, Monitor, MAX_HEADS};
use nonos_intel_display::engine::gen::Gen;
use nonos_intel_display::engine::{error::Error, takeover::Output};

use super::take_head::{run, take_head};
use super::Driver;
use crate::bar::Bar0;
use crate::clock::Uptime;
use crate::error::{IntelError, IntelResult};
use crate::handles::BrokerHandles;

pub fn take(
    h: BrokerHandles,
    mut m: Bar0,
    gen: &'static Gen,
    lit: [Option<Output>; MAX_HEADS],
    total: u32,
    aperture: u64,
) -> IntelResult<Driver> {
    let clock = Uptime;
    let fw = |o: &Output| o.plane.live / 4096 + run(o.plane.stride / 4, o.plane.height);
    let fw_top = lit.iter().flatten().map(fw).max().unwrap_or(0);
    let (mut top, mut first_err) = (total / 64 * 64, IntelError::Engine(Error::NothingLit));
    let mut heads = [None, None, None, None];
    let mut n = 0;
    for out in lit.into_iter().flatten() {
        let monitor = Monitor::of(&mut m, &clock, gen, out.transcoder);
        let p = monitor.panel();
        let big = p.modes[..p.len]
            .iter()
            .fold((out.width, out.height), |a, m| (a.0.max(m.width), a.1.max(m.height)));
        // Two surfaces of the largest mode, and 64 entries for the cursor.
        let Some(floor) = top.checked_sub(2 * run(big.0, big.1) + 64).filter(|&f| f >= fw_top)
        else {
            continue;
        };
        match take_head(&h, &mut m, (&clock, gen), out, (floor, top), &monitor, aperture) {
            Ok(head) => (heads[n], n, top) = (Some(head), n + 1, floor),
            Err(e) if n == 0 => first_err = e,
            Err(_) => {}
        }
    }
    if n == 0 {
        return Err(first_err);
    }
    let mut intel = Intel::new(m, clock, gen, heads);
    intel.watch();
    Ok(Driver { server: Server::new(intel, Pool), handles: h })
}
