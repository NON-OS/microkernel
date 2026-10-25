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

//! A delivery as nonos.prove would build it for the fixture's family while
//! it still serves: the live record, FLAG_FINAL clear; the body
//! from the buyer's values; a device proof bound to ctx2 under e2; and the
//! kind-2 container over them.

use std::sync::OnceLock;

use nonos_proof_abi::{ctx2, delivery_scope, nonce_ref, record_id, req_digest, rsp_digest};
use nonos_proof_abi::{Delivery, Record};

use super::device::{make, Proven, DAY, LISTING};
use super::family;

pub const NONCE: [u8; 32] = [0x6E; 32];
pub const REQUEST: &[u8] = b"what is two and two";
pub const RESPONSE: &[u8] = b"four";
pub const ORDER: [u8; 32] = [0x0D; 32];
pub const CHANNEL: [u8; 32] = [0xC4; 32];

pub struct Delivered {
    pub record: Record,
    pub body: Delivery,
    pub meas: Vec<[u8; 32]>,
    pub policy: [u8; 256],
    pub proven: &'static Proven,
}

pub fn delivered() -> Delivered {
    static ONE: OnceLock<Proven> = OnceLock::new();
    let f = family();
    let record = Record { flags: 0, ..f.record };
    let body = Delivery {
        record_id: record_id(&record.encode()),
        service_meas: record.sup_meas,
        req: req_digest(&NONCE, REQUEST),
        rsp: rsp_digest(&NONCE, RESPONSE),
        order_ref: nonce_ref(&NONCE, &ORDER),
        from: 100,
        to: 200,
        channel_ref: nonce_ref(&NONCE, &CHANNEL),
    };
    let bound = ctx2(&record.encode(), &body.encode());
    let proven = ONE.get_or_init(|| make(bound, delivery_scope(&LISTING, DAY)));
    Delivered { record, body, meas: f.meas, policy: f.policy, proven }
}
