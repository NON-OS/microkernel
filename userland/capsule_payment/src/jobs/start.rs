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

//! Starting a job: OPEN START, TOPUP or CLOSE answer its id at once, or
//! EBUSY while another runs; JOB (op 15) answers its result later.

use nonos_nox_config::{address, Contract};
use nonos_nox_meter::flow::{Contracts, Flow, Work as Job};
use nonos_payment_proto::ENOENT;

use super::drive::advance;
use super::work::{Running, Work};
use crate::meter::{now_ms, Meter};

pub(crate) fn start(work: &mut Work, meter: &mut Meter, job: Job) -> Result<u32, i32> {
    let keys = meter.ready()?;
    let chain = job.chain_id();
    let at = |c| address(chain, c).map_err(|_| ENOENT);
    let contracts =
        Contracts { nox: at(Contract::NoxToken)?, settlement: at(Contract::Settlement)? };
    let id = work.jobs.start()?;
    let (flow, next) = Flow::begin(job);
    let (key_id, address) = (keys.key_id, keys.address);
    work.run = Some(Running { id, flow, contracts, key_id, address, pending: None });
    advance(work, Ok(next), now_ms());
    Ok(id)
}
