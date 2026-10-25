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

//! ORDER_OFFER's proof of work in slices (v1.4.34), the way net.anon's
//! puzzle is walked: SLICE Equi-X indices a pass of the loop, a request
//! served between, so PAY never waits behind an escrow purchase. The job id
//! is taken at the ask; the offer job starts under it once a solution is
//! found, and a walk that gives up fails it ETIME.

use nonos_nox_meter::escrow::Walked;
use nonos_payment_proto::ETIME;

use super::start::start_as;
use super::work::{Work, TICK_MS};
use crate::meter::Meter;

/// Indices a pass: a sixteenth of the space, well under a millisecond.
const SLICE: u32 = 4096;

impl Work {
    /// recv_from's timeout: at once while a walk goes on, each tick while a
    /// step waits, else block (0).
    pub fn wait_ms(&self) -> u64 {
        if self.solving.is_some() {
            return 1;
        }
        TICK_MS * self.run.as_ref().is_some_and(|r| r.pending.is_some()) as u64
    }
}

/// One slice of the running walk, if there is one.
pub(crate) fn solve_step(work: &mut Work, meter: &mut Meter) {
    let Some(s) = work.solving.as_mut() else { return };
    let got = s.walk.step(&mut s.solver, SLICE);
    if matches!(got, Walked::Going) {
        return;
    }
    let Some(s) = work.solving.take() else { return };
    match got {
        Walked::Found(a) => match s.job(a) {
            Ok(job) => start_as(work, meter, job, s.id),
            Err(e) => work.jobs.finish(s.id, Err(e)),
        },
        _ => work.jobs.finish(s.id, Err(ETIME)),
    }
}
