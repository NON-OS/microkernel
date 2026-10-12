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

//! One bring-up attempt: claim, BAR0, the GGTT's size, DC states off and
//! the outputs firmware lit, then those outputs taken over (`take`).
//! Every engine step checks before it writes, and every grant is given back
//! by its owner's drop when a step fails, so the next attempt starts clean.

use nonos_intel_display::engine::access::Until;
use nonos_intel_display::engine::error::Error;
use nonos_intel_display::engine::takeover::read_outputs;
use nonos_intel_display::engine::{dc::dc_off, gen::PlaneKind, ggtt_range::entries};

use super::{claim, maps, take, Driver};
use crate::bar::Bar0;
use crate::clock::Uptime;
use crate::discover::Found;
use crate::error::{IntelError, IntelResult};

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
    let lit = read_outputs(&mut m, gen);
    if lit.iter().all(|o| o.is_none()) {
        return Err(IntelError::Engine(Error::NothingLit));
    }
    take::take(h, m, gen, lit, total, dev.bar2_size)
}
