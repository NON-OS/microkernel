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

//! The delivery digests and ctx2 as plan/14 writes them: each salted by the
//! buyer's nonce, and ctx2 never a record's own ctx.

use nonos_proof_abi::{ctx, ctx2, nonce_ref, req_digest, rsp_digest, Kind};

const NONCE: [u8; 32] = [9; 32];

fn b3(parts: &[&[u8]]) -> [u8; 32] {
    *blake3::hash(&parts.concat()).as_bytes()
}

#[test]
fn every_digest_is_its_domain_the_nonce_and_the_bytes() {
    let req = b3(&[b"NONOS-DELIVERY-REQ-v1", &NONCE, b"ask"]);
    assert_eq!(req_digest(&NONCE, b"ask"), req);
    assert_eq!(rsp_digest(&NONCE, b"ok"), b3(&[b"NONOS-DELIVERY-RSP-v1", &NONCE, b"ok"]));
    assert_eq!(nonce_ref(&NONCE, &[5; 32]), b3(&[&NONCE, &[5; 32]]));
    assert_ne!(req_digest(&[8; 32], b"ask"), req, "another nonce, another digest");
    assert_ne!(nonce_ref(&[8; 32], &[5; 32]), nonce_ref(&NONCE, &[5; 32]));
}

#[test]
fn ctx2_binds_the_record_and_the_body_and_is_no_records_ctx() {
    let (record, body) = ([3u8; 240], [4u8; 208]);
    let d = blake3::hash(
        &[
            &b"NONOS-PROOF-DELIVERY-v1"[..],
            &240u32.to_le_bytes(),
            &record,
            &208u32.to_le_bytes(),
            &body,
        ]
        .concat(),
    );
    assert_eq!(ctx2(&record, &body), nox_verify::attest::digest_words(d.as_bytes()));
    let mut other = body;
    other[207] ^= 1;
    assert_ne!(ctx2(&record, &other), ctx2(&record, &body), "another body");
    assert_ne!(ctx2(&record, &body), ctx(Kind::Delivery, &record), "not the record's ctx");
    assert_ne!(ctx2(&record, &body), ctx(Kind::Isolation, &record));
}
