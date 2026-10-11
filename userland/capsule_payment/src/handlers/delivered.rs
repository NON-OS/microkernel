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

//! ORDER_DELIVERED (op 10, plan/21, plan/29, v1.4.46), from the wallet or
//! the installer. orderId | K: K against the order's keyCommit, then
//! open_delivery over the order's own offer, at once. orderId alone: a job
//! that first reads K through settle's DELIVERED, then does the same. OK
//! seals the plaintext for op 18 and marks the order plain; a bad chunk
//! answers dispute()'s calldata for the wallet's Dispute intent.

use alloc::vec::Vec;

use nonos_nox_meter::flow::Work as Job;
use nonos_nox_meter::orders::{deliver, Opened, Order};
use nonos_payment_proto::{job_id, DeliveredAsk, DeliveredMade, EINVAL};

use crate::jobs::{start, Work};
use crate::keccak::Host;
use crate::meter::Meter;
use crate::orders::{escrow_of, find, keep, keep_plain};
use crate::server::{status_of, Call};

pub fn order_delivered(meter: &mut Meter, work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    if let Ok(order_id) = <[u8; 32]>::try_from(req.body) {
        let job = find(&order_id).map(Job::Deliver);
        return req.result(
            job.and_then(|j| start(work, meter, j, req.sender)).map(|id| job_id(id).to_vec()),
        );
    }
    let id = match work.jobs.start(req.sender) {
        Ok(id) => id,
        Err(e) => return req.answer(e, &[]),
    };
    let r =
        DeliveredAsk::parse(req.body).ok_or(EINVAL).and_then(|a| deliver_key(&a.order_id, &a.key));
    work.jobs.finish(id, r);
    req.result(Ok(job_id(id).to_vec()))
}

/// The delivery opened with `key`: the plaintext kept, or the bad chunk.
pub(crate) fn deliver_key(order_id: &[u8; 32], key: &[u8; 32]) -> Result<Vec<u8>, i32> {
    let order = find(order_id)?;
    let kept = escrow_of(&order)?;
    match deliver(&Host, &order, &kept, key).map_err(status_of)? {
        Opened::Plain(plain) => {
            keep_plain(&order.order_id, plain)?;
            keep(&Order { plain: true, ..order })?;
            Ok(DeliveredMade::Ok { content_len: kept.content_len }.encode())
        }
        Opened::Bad(index, calldata) => Ok(DeliveredMade::BadChunk { index, calldata }.encode()),
    }
}
