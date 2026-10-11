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

//! ORDER_DELIVERED asked again (v1.4.49): a plain order answers OK from its
//! escrow record, a bad chunk this boot kept answers that, and only an
//! order with neither opens its delivery again.

use alloc::vec::Vec;

use nonos_nox_meter::orders::KeptDelivered;
use nonos_payment_proto::{job_id, DeliveredMade};

use crate::jobs::Work;
use crate::orders::{escrow_of, find};
use crate::server::Call;

/// The answer already made for the order, if any.
pub(super) fn answered(kept: &KeptDelivered, order_id: &[u8; 32]) -> Option<Result<Vec<u8>, i32>> {
    if let Some(a) = kept.get(order_id) {
        return Some(Ok(a.to_vec()));
    }
    let order = find(order_id).ok().filter(|o| o.plain)?;
    Some(escrow_of(&order).map(|e| DeliveredMade::Ok { content_len: e.content_len }.encode()))
}

/// A job that ends at once with what `r` makes, made only once it started.
pub(super) fn finished<F>(work: &mut Work, req: &Call<'_>, r: F) -> Vec<u8>
where
    F: FnOnce(&mut Work) -> Result<Vec<u8>, i32>,
{
    match work.jobs.start(req.sender) {
        Ok(id) => {
            let r = r(work);
            work.jobs.finish(id, r);
            req.result(Ok(job_id(id).to_vec()))
        }
        Err(e) => req.answer(e, &[]),
    }
}
