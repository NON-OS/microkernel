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

//! ORDER_OFFER (op 17, the plan), from the wallet alone. The meter draws the
//! buyer key and a nonce start, walks nonces to the proof of work, and
//! starts the job that sends FETCH 1 ESCROW and pages the answer. An offer
//! whose sealed answer could pass MAX_RELEASE is refused before any work.

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonos_equix::Solver;
use nonos_nox_chunks::offer_len;
use nonos_nox_meter::escrow::{escrow_request, OfferWork, Want};
use nonos_nox_meter::flow::Work as Job;
use nonos_payment_proto::{OfferAsk, EAGAIN, EINVAL, ETIME};
use nonos_settle_proto::{Fetch, MAX_RELEASE};

use super::channel_ops::started;
use crate::jobs::Work;
use crate::keys::{random, Dalek};
use crate::meter::{now_ms, Meter};
use crate::server::Call;

pub fn order_offer(meter: &mut Meter, work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    started(work, meter, OfferAsk::parse(req.body).ok_or(EINVAL).and_then(|a| asked(&a)), req)
}

fn asked(a: &OfferAsk) -> Result<Job, i32> {
    let sealed = offer_len(a.content_len).and_then(|n| n.checked_add(48)).ok_or(EINVAL)?;
    (sealed <= MAX_RELEASE).then_some(()).ok_or(EINVAL)?;
    let want = Want {
        chain_id: a.chain_id,
        url: a.url.clone(),
        listing_key: a.listing_key,
        release_key: a.release_key,
    };
    let mut solver = Solver::new().ok_or(EAGAIN)?;
    let mut draw = || {
        let mut b = [0u8; 32];
        random(&mut b).ok().map(|_| b)
    };
    let asked = escrow_request(&Dalek, &mut draw, &mut solver, &want).ok_or(ETIME)?;
    let buyer_pk = asked.request.buyer_pk;
    let request = Fetch::Escrow(asked.request).encode().ok_or(EINVAL)?;
    Ok(Job::Offer(Box::new(OfferWork {
        want,
        chunk_root: a.chunk_root,
        content_len: a.content_len,
        buyer_sk: asked.buyer_sk,
        buyer_pk,
        request,
        made_at: now_ms() / 1000,
        fetched: Vec::new(),
    })))
}
