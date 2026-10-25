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

//! reset.rs's marker and its reset writes: INTx Disable set and read back,
//! else Memory Space where a memory BAR decodes; the reset by the method
//! chosen, Command through the lock every edit of it takes (L73); and
//! act.rs `to_d0`, a power cycle's way back.

use super::bits_rule::{pmcsr_to, D3HOT, INITIATE_FLR, INTX_DISABLE, MEM};
use super::marker_rule::{marker, Marker};
use super::method_rule::Method;
use super::phase_rule::Phase;
use super::start_rule::written;
use super::world::World;
use super::world_types::Wipe;

pub(super) fn mark(w: &mut World) -> Option<Marker> {
    let sets = |w: &mut World, bit: u16| {
        w.update_command(true, |v| v | bit);
        w.command() & bit != 0
    };
    let intx = sets(w, INTX_DISABLE);
    let mem = !intx && w.dev.caps.mem_bar && sets(w, MEM);
    marker(intx, mem)
}

pub(super) fn write(w: &mut World, mut job: Wipe, m: Method, now: u64) -> Wipe {
    match m {
        Method::Flr => {
            let d = w.devctl();
            w.set_devctl(true, d | INITIATE_FLR);
        }
        Method::AfFlr => w.af_flr(),
        Method::PmCycle => {
            let p = w.pmcsr();
            w.set_pmcsr(true, pmcsr_to(p, D3HOT));
        }
    }
    job.method = Some(m);
    job.phase = written(m, now);
    job
}

/// act.rs `to_d0`: back to D0, which resets a function that keeps no state
/// through D3hot (recalled, unverified).
pub(super) fn to_d0(w: &mut World, mut job: Wipe, now: u64) -> Wipe {
    let p = w.pmcsr();
    w.set_pmcsr(true, pmcsr_to(p, 0));
    job.phase = Phase::D0 { at: now };
    job
}
