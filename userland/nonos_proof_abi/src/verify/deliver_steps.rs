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

//! Steps 2, 3 and 5 of a delivery's verify (plan/15): a live record of a
//! pinned vendor supervisor running this policy's app; a body naming that
//! record, that supervisor and the buyer's own values; the measurements
//! rebuilding the record's M.

use super::bind::app_id;
use super::deliver_expected::DeliveryExpected;
use super::refusal::Refusal;
use crate::delivery::{nonce_ref, req_digest, rsp_digest, Delivery};
use crate::meas::meas_chain;
use crate::record::Record;
use crate::trust::supervisor_caps;

/// Not final, saturated or overflowed: a family still serving.
pub(super) fn live_record(rec: &Record, ex: &DeliveryExpected, mask: u64) -> Result<(), Refusal> {
    let vendor = rec.authority == 0
        && ex.pins.supervisors.contains(&rec.sup_meas)
        && rec.sup_caps == supervisor_caps(mask);
    if rec.flags != 0 || !vendor {
        return Err(Refusal::Supervisor);
    }
    let program: &[u8; 32] = ex.policy[16..48].try_into().map_err(|_| Refusal::Policy)?;
    if rec.app_id != app_id(ex.policy, program) {
        return Err(Refusal::AppId);
    }
    Ok(())
}

pub(super) fn body(
    b: &Delivery,
    rec: &Record,
    id: &[u8; 32],
    ex: &DeliveryExpected,
) -> Result<(), Refusal> {
    let n = &ex.nonce;
    let ours = b.service_meas == rec.sup_meas
        && b.record_id == *id
        && b.req == req_digest(n, ex.request)
        && b.rsp == rsp_digest(n, ex.response)
        && b.order_ref == nonce_ref(n, &ex.order_id)
        && b.channel_ref == nonce_ref(n, &ex.channel_id);
    ours.then_some(()).ok_or(Refusal::Body)
}

/// The list rebuilds M exactly, entry for entry.
pub(super) fn measurements(list: &[u8], rec: &Record) -> Result<(), Refusal> {
    let entries: alloc::vec::Vec<[u8; 32]> =
        list.chunks_exact(32).filter_map(|c| c.try_into().ok()).collect();
    let whole = entries.len() as u64 == u64::from(rec.n_meas);
    (whole && meas_chain(&entries) == rec.meas).then_some(()).ok_or(Refusal::Measurements)
}
