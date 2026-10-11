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

//! The delivery the job builds from a live family (plan/15): the record is
//! held to be vendor and live; the body is made of the prover's own digests;
//! e2 and ctx2 are the abi's.

use nonos_proof_abi::wire::{NOT_VENDOR, PROVER};
use nonos_proof_abi::{ctx2, delivery_scope, nonce_ref, record_id};
use nonos_proof_abi::{req_digest, rsp_digest, Delivery, Record};

use crate::fixture::{ask, live, LISTING};
use crate::market::delivery::{body_of, delivery_bound, live_check};

const DAY: u64 = 20_372;

#[test]
fn a_record_must_be_vendor_and_live() {
    let (s, _) = live();
    assert_eq!(live_check(&s.record), Ok(()));
    assert_eq!(live_check(&Record { authority: 1, ..s.record }), Err(NOT_VENDOR));
    assert_eq!(live_check(&Record { authority: 255, ..s.record }), Err(NOT_VENDOR));
    assert_eq!(live_check(&Record { flags: 1, ..s.record }), Err(PROVER), "final");
    assert_eq!(live_check(&Record { flags: 2, ..s.record }), Err(PROVER), "saturated");
}

#[test]
fn the_body_is_the_provers_own_digests_and_e2_ctx2_the_abis() {
    let (s, p) = live();
    let b = Delivery::decode(&body_of(&s, &ask(&p, b"ask", b"answer"))).expect("a body");
    assert_eq!(
        (b.record_id, b.service_meas, b.from, b.to),
        (record_id(&s.raw), s.record.sup_meas, 1, 2)
    );
    let n = &[0x6E; 32];
    assert_eq!((b.req, b.rsp), (req_digest(n, b"ask"), rsp_digest(n, b"answer")));
    assert_eq!(
        (b.order_ref, b.channel_ref),
        (nonce_ref(n, &[0x0D; 32]), nonce_ref(n, &[0xC4; 32]))
    );
    let body = b.encode();
    let (e, c) = delivery_bound(&LISTING, DAY, &s, &body);
    assert_eq!(e.map(|f| f.to_u64()), delivery_scope(&LISTING, DAY));
    assert_eq!(c.map(|f| f.to_u64()), ctx2(&s.raw, &body));
}
