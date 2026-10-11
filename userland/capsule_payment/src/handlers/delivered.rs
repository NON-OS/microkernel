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

//! ORDER_DELIVERED (op 10, the plan, the plan), from the wallet or the
//! installer: K against the order's keyCommit, then open_delivery over the
//! order's own offer. OK seals the plaintext for op 18; a bad chunk answers
//! dispute()'s calldata for the wallet's Dispute intent. It needs nothing of
//! the chain, so the job ends as it starts, and JOB answers its caller alone.

use alloc::vec::Vec;

use nonos_nox_meter::orders::{deliver, Opened, Order};
use nonos_payment_proto::{job_id, DeliveredAsk, DeliveredMade, EINVAL};

use crate::jobs::Work;
use crate::keccak::Host;
use crate::orders::{escrow_of, find, keep, keep_plain};
use crate::server::{status_of, Call};

pub fn order_delivered(work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    let id = match work.jobs.start(req.sender) {
        Ok(id) => id,
        Err(e) => return req.answer(e, &[]),
    };
    work.jobs.finish(id, delivered(req.body));
    req.result(Ok(job_id(id).to_vec()))
}

fn delivered(body: &[u8]) -> Result<Vec<u8>, i32> {
    let a = DeliveredAsk::parse(body).ok_or(EINVAL)?;
    let order = find(&a.order_id)?;
    let kept = escrow_of(&order)?;
    match deliver(&Host, &order, &kept, &a.key).map_err(status_of)? {
        Opened::Plain(plain) => {
            keep_plain(&order.order_id, plain)?;
            keep(&Order { plain: true, ..order })?;
            Ok(DeliveredMade::Ok { content_len: kept.content_len }.encode())
        }
        Opened::Bad(index, calldata) => Ok(DeliveredMade::BadChunk { index, calldata }.encode()),
    }
}
