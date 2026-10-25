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

//! Starting a job: OPEN START, TOPUP, CLOSE or CLAIM answer its id at once, or
//! EBUSY while another runs; JOB (op 15) answers its result later. An offer
//! takes its id at the ask and starts here once its proof of work is found.

use nonos_nox_config::{address, Contract};
use nonos_nox_meter::flow::{Contracts, Flow, Work as Job};
use nonos_payment_proto::ENOENT;

use super::drive::advance;
use super::work::{Running, Work};
use crate::keys::Keys;
use crate::meter::{now_ms, Meter};

/// Start `job` for `owner`, the pid alone that JOB then answers.
pub(crate) fn start(work: &mut Work, meter: &mut Meter, job: Job, owner: u32) -> Result<u32, i32> {
    let ready = prepare(meter, &job)?;
    let id = work.jobs.start(owner)?;
    run(work, ready, job, id);
    Ok(id)
}

/// Start `job` under `id`, taken earlier; what stops it fails that id.
pub(super) fn start_as(work: &mut Work, meter: &mut Meter, job: Job, id: u32) {
    match prepare(meter, &job) {
        Ok(ready) => run(work, ready, job, id),
        Err(e) => work.jobs.finish(id, Err(e)),
    }
}

fn prepare(meter: &mut Meter, job: &Job) -> Result<(Keys, Contracts), i32> {
    let keys = meter.ready()?;
    let chain = job.chain_id();
    for c in job.needs() {
        address(chain, *c).map_err(|_| ENOENT)?;
    }
    let at = |c| address(chain, c).unwrap_or([0; 20]);
    let (nox, settlement) = (at(Contract::NoxToken), at(Contract::Settlement));
    Ok((keys, Contracts { nox, settlement, pool: at(Contract::ShieldPool) }))
}

fn run(work: &mut Work, (keys, contracts): (Keys, Contracts), job: Job, id: u32) {
    let (flow, next) = Flow::begin(job);
    let (key_id, address) = (keys.key_id, keys.address);
    work.run = Some(Running { id, flow, contracts, key_id, address, pending: None });
    advance(work, Ok(next), now_ms());
}
