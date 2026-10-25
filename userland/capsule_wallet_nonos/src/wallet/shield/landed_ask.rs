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
//! The landed reads' two requests for one address (v1.4.55, v1.4.56): NOX's
//! balanceOf through READ, then its ETH through GAS, each ASYNC to settle.

use nonos_settle_proto::{parse_read_reply, Gas, Read, OP_GAS, OP_READ};

use super::landed::balance_of;
use super::landed_held::Reading;
use crate::wallet::settle::{SettleCall, LANDED};

const BALANCE_OF: u16 = 1;

/// The op this phase of `r` asks: READ for NOX, then GAS.
pub fn op_of(r: &Reading) -> u16 {
    if r.nox.is_none() {
        OP_READ
    } else {
        OP_GAS
    }
}

/// Ask this phase's request for `r`. Whether it went out.
pub fn ask(settle: &mut SettleCall, r: &Reading, chain_id: u64, now: u64) -> bool {
    let body = match op_of(r) {
        OP_READ => {
            let mut w = [0u8; 32];
            w[12..].copy_from_slice(&r.address);
            Read { chain_id, view: BALANCE_OF, args: [w].into() }.encode()
        }
        _ => Some(Gas::request(chain_id, &r.address).to_vec()),
    };
    body.is_some_and(|b| settle.ask(op_of(r), LANDED, &b, now).is_ok())
}

/// The answer to this phase, read as wei; None when it did not answer.
pub fn answered(op: u16, r: Result<alloc::vec::Vec<u8>, i32>) -> Option<u128> {
    let b = r.ok()?;
    match op {
        OP_READ => parse_read_reply(&b).and_then(|w| balance_of(&w)),
        _ => Gas::parse(&b).map(|g| g.balance),
    }
}
