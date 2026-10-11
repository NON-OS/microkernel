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

//! TOPUP (op 13), CLOSE (op 14) and JOB (op 15), from the wallet (plan/21).
//! TOPUP and CLOSE start a job and answer its id at once; the job asks
//! settle for GAS (and, to close, GRACE), signs with the meter's own key
//! and has settle broadcast, each step ASYNC (v1.4.15). JOB answers EAGAIN
//! while it runs, then the hashes once. Neither changes the meter's state:
//! the next CHANNEL_STATE reads the new deposit, or the close.

use alloc::vec::Vec;

use nonos_nox_meter::flow::Work as Job;
use nonos_payment_proto::{job_id, parse_job_id, Close, TopUp, EINVAL};

use crate::jobs::{start, Work};
use crate::meter::Meter;
use crate::server::Call;

pub(super) fn started(
    work: &mut Work,
    meter: &mut Meter,
    job: Result<Job, i32>,
    req: &Call<'_>,
) -> Vec<u8> {
    let r = job.and_then(|j| start(work, meter, j, req.sender));
    req.result(r.map(|id| job_id(id).to_vec()))
}

pub fn topup(meter: &mut Meter, work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    started(work, meter, TopUp::parse(req.body).map(Job::TopUp).ok_or(EINVAL), req)
}

pub fn close(meter: &mut Meter, work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    started(work, meter, Close::parse(req.body).map(Job::Close).ok_or(EINVAL), req)
}

pub fn open_start(meter: &mut Meter, work: &mut Work, job: Job, req: &Call<'_>) -> Vec<u8> {
    started(work, meter, Ok(job), req)
}

/// CLAIM (op 16, v1.4.17): body chain_id u64; the meter claims to itself.
pub fn claim(meter: &mut Meter, work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    let chain = req.body.try_into().ok().map(u64::from_le_bytes);
    let at = meter.ready().map(|k| k.address).ok();
    let job = chain.zip(at).map(|(chain_id, meter)| Job::Claim { chain_id, meter }).ok_or(EINVAL);
    started(work, meter, job, req)
}

pub fn job(work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    req.result(parse_job_id(req.body).ok_or(EINVAL).and_then(|id| work.jobs.take(id, req.sender)))
}
