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

//! ORDER_OFFER (op 17, plan/29), from the wallet alone. The meter draws the
//! buyer key and a nonce start and answers the job id at once; the walk to
//! the proof of work goes on in slices between requests (jobs/solve.rs),
//! then the job sends FETCH 1 ESCROW and pages the answer. An offer whose
//! sealed answer could pass MAX_RELEASE is refused before any work.

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonos_equix::Solver;
use nonos_nox_chunks::offer_len;
use nonos_nox_config::Chain;
use nonos_nox_meter::escrow::{Walk, Want};
use nonos_payment_proto::{job_id, OfferAsk, EAGAIN, EINVAL};
use nonos_settle_proto::MAX_RELEASE;

use crate::jobs::{Solving, Work};
use crate::keys::{random, Dalek};
use crate::meter::Meter;
use crate::server::Call;

pub fn order_offer(meter: &mut Meter, work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    let r = OfferAsk::parse(req.body).ok_or(EINVAL).and_then(|a| asked(meter, work, &a, req));
    req.result(r.map(|id| job_id(id).to_vec()))
}

fn asked(meter: &mut Meter, work: &mut Work, a: &OfferAsk, req: &Call<'_>) -> Result<u32, i32> {
    let sealed = offer_len(a.content_len).and_then(|n| n.checked_add(48)).ok_or(EINVAL)?;
    (sealed <= MAX_RELEASE).then_some(()).ok_or(EINVAL)?;
    Chain::from_id(a.chain_id).ok_or(EINVAL)?;
    meter.ready()?;
    let want = Want {
        chain_id: a.chain_id,
        url: a.url.clone(),
        listing_key: a.listing_key,
        release_key: a.release_key,
    };
    let solver = Box::new(Solver::new().ok_or(EAGAIN)?);
    let (mut buyer_sk, mut start) = ([0u8; 32], [0u8; 16]);
    random(&mut buyer_sk).and_then(|_| random(&mut start)).map_err(|_| EAGAIN)?;
    let walk = Walk::new(&Dalek, buyer_sk, start, want);
    crate::wipe::wipe(&mut buyer_sk);
    let id = work.jobs.start(req.sender)?;
    let (chunk_root, content_len) = (a.chunk_root, a.content_len);
    work.solving = Some(Box::new(Solving { id, walk, solver, chunk_root, content_len }));
    Ok(id)
}
