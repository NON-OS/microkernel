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

use nonos_proof_abi::{meas_chain, meas_step, nonce_ref, req_digest, rsp_digest, Delivery};

const NONCE: [u8; 32] = [9; 32];

#[test]
fn a_delivery_body_reads_back_as_written() {
    let d = Delivery {
        record_id: [1; 32],
        service_meas: [2; 32],
        req: req_digest(&NONCE, b"q"),
        rsp: rsp_digest(&NONCE, b"a"),
        order_ref: nonce_ref(&NONCE, &[5; 32]),
        from: 10,
        to: u64::MAX,
        channel_ref: nonce_ref(&NONCE, &[6; 32]),
    };
    let b = d.encode();
    assert_eq!(Delivery::decode(&b), Some(d));
    assert_eq!(Delivery::decode(&b[..207]), None);
    assert_eq!(&b[128..160], &d.order_ref);
    assert_eq!(&b[176..], &d.channel_ref);
    assert_ne!(req_digest(&NONCE, b"x"), rsp_digest(&NONCE, b"x"), "a request is never a response");
}

#[test]
fn the_measurement_chain_keeps_its_order() {
    let (a, b) = ([1u8; 32], [2u8; 32]);
    assert_eq!(meas_chain(&[]), [0; 32]);
    assert_eq!(meas_chain(&[a, b]), meas_step(&meas_step(&[0; 32], &a), &b));
    assert_ne!(meas_chain(&[a, b]), meas_chain(&[b, a]));
    let mut want = b"NONOS-FAMILY-MEAS-v1".to_vec();
    want.extend_from_slice(&[0; 32]);
    want.extend_from_slice(&a);
    assert_eq!(meas_chain(&[a]), *blake3::hash(&want).as_bytes());
}
