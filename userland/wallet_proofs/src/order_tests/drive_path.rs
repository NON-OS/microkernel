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
//! A refund's path through the step machine, from orderOf to its broadcast.

use nonos_nox_config::REFUND_GAS;
use nonos_settle_proto::{read_reply, Gas, Quote};

use super::dispute_fixture::{quote, word};
use super::drive_rig::Rig;
use crate::order::send_step::{SendStep, REFUND};
use crate::wallet::settle::send::{SENT, SETTLE_PID};

pub(super) const RAW: [u8; 4] = [0x02, 0xf8, 0x71, 0x99];

pub(super) fn sent(op: u16) -> Vec<Vec<u8>> {
    SENT.with(|s| s.borrow().iter().filter(|f| f.0 == op).map(|f| f.2.clone()).collect())
}

/// A refund of a Disputed order to its broadcast, as the Shield signs it.
pub(super) fn to_broadcast(r: &mut Rig) {
    r.tick();
    assert!(r.answer(SETTLE_PID, 0, &order_of(3)) && r.tick());
    assert!(matches!(r.step(), SendStep::Quoting(_)));
    r.tick();
    let q = Quote { kind: REFUND, gas_limit: REFUND_GAS, ..quote() };
    assert!(r.answer(SETTLE_PID, 0, &q.encode()) && r.tick());
    assert!(matches!(r.step(), SendStep::Gas { .. }), "{:?}", r.step());
    r.now_ms += 10_000;
    r.tick();
    let g =
        Gas { balance: u128::MAX, tx_nonce: 0, max_fee_per_gas: 1, max_priority_fee_per_gas: 1 };
    assert!(r.answer(SETTLE_PID, 0, &g.encode()) && r.tick());
    assert!(matches!(r.step(), SendStep::Signing(_)));
    (r.e.raw, r.e.step) = (Some(RAW.to_vec()), SendStep::Broadcasting(RAW.to_vec()));
    r.tick();
}

/// orderOf's five words with `state`.
pub(super) fn order_of(state: u64) -> Vec<u8> {
    read_reply(&[word(state), word(9), word(0), word(0), word(0)]).unwrap()
}
