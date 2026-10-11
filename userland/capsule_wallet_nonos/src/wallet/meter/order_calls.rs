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

/*
 * A purchase's calls to the meter (B8, the plan, the plan): TAKE from the
 * store's mailbox, and JOB for an order's jobs, whose results run longer
 * than a channel's: ORDER_NEW answers the deposit address and 14 words.
 */

use alloc::vec::Vec;

use nonos_payment_proto::{Names, MAX_NAME, OP_JOB, OP_PURCHASE, PURCHASE_TAKE};

use super::call::call;

/// The names the store posted, once; None when none wait, or the meter
/// did not answer and is asked again on a later tick.
pub fn take() -> Option<Names> {
    let reply = call(OP_PURCHASE, &[PURCHASE_TAKE], 8 + 2 * (2 + MAX_NAME)).ok()?;
    Names::parse(&reply)
}

/// 19/2 READY: the names and their orderId, held for the store.
pub fn ready(r: &nonos_payment_proto::Ready) -> Result<(), i32> {
    let body = r.post().ok_or(nonos_payment_proto::EINVAL)?;
    call(OP_PURCHASE, &body, 0).map(|_| ())
}

/// LISTING_CAP (op 20): one listing's cap, the rest of the budget kept.
pub fn listing_cap(listing_id: [u8; 32], cap: u128) -> Result<(), i32> {
    let body = nonos_payment_proto::ListingCapAsk { listing_id, cap }.encode();
    call(nonos_payment_proto::OP_LISTING_CAP, &body, 0).map(|_| ())
}

/// OPEN CONFIRM: the channel id once its opening is mined, EAGAIN before.
pub fn open_confirm(body: &[u8]) -> Result<Vec<u8>, i32> {
    call(nonos_payment_proto::OP_OPEN, body, 32)
}

/// JOB(id) for a purchase's job: EAGAIN while it runs, then its result,
/// at most JOB_MAX: a bad chunk's dispute calldata runs past ORDER_NEW's.
pub fn order_job(id: u32) -> Result<Vec<u8>, i32> {
    call(OP_JOB, &id.to_le_bytes(), JOB_MAX)
}

const JOB_MAX: usize = 3_968;
