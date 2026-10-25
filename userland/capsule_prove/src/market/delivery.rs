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

//! A delivery from a live family (plan/15): a vendor supervisor's live, whole
//! record; a body whose every digest is the prover's own; ctx2 under e2; the
//! kind-2 container over sections 1, 2, 6 and 7.

use alloc::vec::Vec;

use nonos_proof_abi::wire::{DeliverRequest, NOT_VENDOR, PROVER};
use nonos_proof_abi::{ctx2, delivery_scope, encode, nonce_ref, record_id, req_digest};
use nonos_proof_abi::{rsp_digest, tag, Delivery, Kind, Record, DELIVERY_BYTES};
use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

use super::sealed::Sealed;

/// Authority 0 and no flag set: not final, saturated or overflowed.
pub fn live_check(rec: &Record) -> Result<(), i32> {
    if rec.authority != 0 {
        return Err(NOT_VENDOR);
    }
    (rec.flags == 0).then_some(()).ok_or(PROVER)
}

pub fn body_of(s: &Sealed, req: &DeliverRequest) -> [u8; DELIVERY_BYTES] {
    let n = req.nonce;
    Delivery {
        record_id: record_id(&s.raw),
        service_meas: s.record.sup_meas,
        req: req_digest(n, req.request),
        rsp: rsp_digest(n, req.response),
        order_ref: nonce_ref(n, req.order_id),
        from: req.from,
        to: req.to,
        channel_ref: nonce_ref(n, req.channel_id),
    }
    .encode()
}

/// e2 and ctx2: only the prover derives them, from the listing, the day,
/// the live record and the body.
pub fn delivery_bound(
    listing_id: &[u8; 32],
    day: u64,
    s: &Sealed,
    body: &[u8; DELIVERY_BYTES],
) -> ([Fp; 2], [Fp; RATE]) {
    let e = delivery_scope(listing_id, day).map(Fp::from_u64);
    (e, ctx2(&s.raw, body).map(Fp::from_u64))
}

pub fn container2(day: u64, s: &Sealed, device: &[u8], body: &[u8]) -> Option<Vec<u8>> {
    let list = s.list.concat();
    let sections: [(u16, &[u8]); 4] = [
        (tag::RECORD, &s.raw),
        (tag::DEVICE, device),
        (tag::DELIVERY, body),
        (tag::MEASUREMENTS, &list),
    ];
    encode(Kind::Delivery, day, &sections)
}
