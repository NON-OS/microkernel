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

//! The seal number the Terminal acks is read from the receipt's record where
//! the NFSR v1 layout holds it, so an ack names the record the receipt is of.

use agentproof_receipt::napr::write;
use nonos_proof_abi::wire::{self, Header, PROVE_MAGIC, RECEIPT_ACK};
use nonos_proof_abi::Record;

use crate::terminal_receipt_frame as t;

#[test]
fn the_ack_names_the_records_own_seal_number() {
    let rec = Record {
        flags: 1,
        sup_meas: [1; 32],
        sup_caps: 3,
        authority: 0,
        seal_no: 0x1122_3344_5566_7788,
        trace: [2; 32],
        rows: 4,
        hc: [5; 4],
        meas: [6; 32],
        n_meas: 7,
        app_id: [8; 32],
    };
    let napr = write::finish(write::bound(&rec.encode(), b"strict", &[0; 256], &[0; 32]), b"d");
    assert_eq!(t::seal_no(&napr), Some(rec.seal_no));
    let ack = t::request(RECEIPT_ACK, 1, &rec.seal_no.to_le_bytes());
    let (_, body) = Header::parse(&ack, PROVE_MAGIC).unwrap();
    assert_eq!(wire::ack_seal_no(body), Some(rec.seal_no));
    let renamed = [&b"NAPX"[..], &napr[4..]].concat();
    let v1 = [&napr[..5], &[1], &napr[6..]].concat();
    assert_eq!([&napr[..60], &renamed, &v1].map(|b| t::seal_no(b)), [None; 3]);
}
