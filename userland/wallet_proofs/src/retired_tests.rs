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

//! Ops 11 and 12 signed for the retired ReceiptSettlement proxy; 13 and 18
//! signed plain transfers no caller asked for. The keyring answers all four
//! EINVAL before any handler, and keeps their numbers.

use crate::server::protocol::retired::{retired, RETIRED};
use crate::server::protocol::types::{Request, OP_SIGN_ETH_TRANSFER, OP_SIGN_NOX_TRANSFER};
use crate::server::protocol::types::{OP_SIGN_NOX_APPROVE, OP_SIGN_NOX_RECEIPT};

const DISPATCH: &str = include_str!("../../capsule_keyring/src/server/dispatch.rs");

/// The response frame: seq u32 LE, then status i32 LE, then the payload.
fn status(frame: &[u8]) -> i32 {
    i32::from_le_bytes(frame[4..8].try_into().unwrap())
}

#[test]
fn the_retired_ops_answer_einval() {
    for op in [OP_SIGN_NOX_RECEIPT, OP_SIGN_NOX_APPROVE, OP_SIGN_ETH_TRANSFER, OP_SIGN_NOX_TRANSFER]
    {
        let req = Request { seq: 7, op, payload: &[1; 200] };
        let frame = retired(&req).expect("retired");
        assert_eq!((&frame[..4], status(&frame), frame.len()), (&7u32.to_le_bytes()[..], -22, 8));
    }
    assert_eq!(RETIRED, [11, 12, 13, 18]);
}

#[test]
fn every_other_op_reaches_its_handler() {
    for op in (0..64).filter(|op| !RETIRED.contains(op)) {
        assert!(retired(&Request { seq: 1, op, payload: &[] }).is_none(), "op {op}");
    }
    let first = DISPATCH.find("protocol::retired(&req)").expect("dispatch asks first");
    assert!(first < DISPATCH.find("match req.op").unwrap());
    for gone in ["sign_receipt", "sign_approve", "sign_eth_transfer", "sign_nox_transfer"] {
        assert!(!DISPATCH.contains(gone), "{gone} has no handler");
    }
}
