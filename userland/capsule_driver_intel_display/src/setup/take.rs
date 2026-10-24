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
//! a picture firmware is still scanning out; a dead driver's leftover is
//! mapped over (`take_plan`). An output that does not fit, or fails, is
//! left as it was lit; none at all fails the attempt. Then whatever a
//! killed driver left is let go of, the GGTT rewritten whole (`settle`).

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;
use nonos_intel_display::display::{settle_heads, Intel, FLIP_US, MAX_HEADS};
use nonos_intel_display::engine::access::Until;
use nonos_intel_display::engine::gen::Gen;
use nonos_intel_display::engine::{error::Error, takeover::Output};

use super::take_head::take_head;
use super::take_plan::plan;
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
    let p = plan(&mut m, &clock, gen, lit, total);
    let (mut top, mut first_err) = (total / 64 * 64, IntelError::Engine(Error::NothingLit));
    let mut heads = [None, None, None, None];
    let mut n = 0;
    for (out, monitor, need) in p.outs.into_iter().flatten() {
        let Some(floor) = top.checked_sub(need).filter(|&f| f >= p.fw_top) else { continue };
        let at = (floor, top, p.ours);
        match take_head(&h, &mut m, (&clock, gen), out, at, &monitor, aperture) {
            Ok(head) => (heads[n], n, top) = (Some(head), n + 1, floor),
            Err(e) if n == 0 => first_err = e,
            Err(_) => {}
        }
    }
    if n == 0 {
        return Err(first_err);
    }
    let until = Until::new(&clock, FLIP_US);
    settle_heads(&mut m, &lit, &heads, (gen.ggtt, p.ours, total), &until);
    let mut intel = Intel::new(m, clock, gen, heads);
    intel.watch();
    Ok(Driver { server: Server::new(intel, Pool::default()), handles: h })
}
