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

use nonos_nox_meter::open::approve_data;
use nonos_nox_meter::settle::{close_data, close_plan, topup_data, topup_plan};
use nonos_nox_meter::MeterError;
use nonos_payment_proto::{Close, TopUp};
use nonos_settle_proto::Gas;

use super::fixture::{gas, state, CHAIN, NOW, NOX, SETTLEMENT};

#[test]
fn a_topup_is_approve_then_topup_at_the_next_two_nonces() {
    let t = TopUp { chain_id: CHAIN, channel_id: [1; 32], amount: 500 };
    let [a, b] = topup_plan(&state(), &t, (&NOX, &SETTLEMENT), &gas(), NOW).unwrap();
    assert_eq!(
        (a.to, a.nonce, a.gas, a.data),
        (NOX, 7, 60_000, approve_data(&SETTLEMENT, 500).to_vec())
    );
    assert_eq!(
        (b.to, b.nonce, b.gas, b.data),
        (SETTLEMENT, 8, 120_000, topup_data(&[1; 32], 500).to_vec())
    );
    let refused =
        |t: TopUp, g: Gas, now| topup_plan(&state(), &t, (&NOX, &SETTLEMENT), &g, now).err();
    assert_eq!(refused(TopUp { amount: 0, ..t }, gas(), NOW), Some(MeterError::Mismatch));
    assert_eq!(
        refused(TopUp { channel_id: [9; 32], ..t }, gas(), NOW),
        Some(MeterError::NoChannel)
    );
    assert_eq!(refused(t, gas(), NOW + 31 * 86_400), Some(MeterError::Expired));
    let short = Gas { balance: 180_000 * 10 - 1, ..gas() };
    assert_eq!(refused(t, short, NOW), Some(MeterError::NoGas));
    let tipped = Gas { max_priority_fee_per_gas: 11, ..gas() };
    assert_eq!(refused(t, tipped, NOW), Some(MeterError::NoGas));
}

#[test]
fn a_close_waits_for_expiry_plus_grace() {
    let c = Close { chain_id: CHAIN, channel_id: [1; 32] };
    let expiry = state().channels[0].expiry;
    let grace = 3_600;
    let early = close_plan(&state(), &c, &SETTLEMENT, &gas(), (expiry + grace - 1, grace));
    assert_eq!(early.err(), Some(MeterError::TooEarly));
    let tx = close_plan(&state(), &c, &SETTLEMENT, &gas(), (expiry + grace, grace)).unwrap();
    assert_eq!(
        (tx.to, tx.nonce, tx.gas, tx.data),
        (SETTLEMENT, 7, 120_000, close_data(&[1; 32]).to_vec())
    );
    let other = Close { channel_id: [9; 32], ..c };
    let unknown = close_plan(&state(), &other, &SETTLEMENT, &gas(), (expiry + grace, grace));
    assert_eq!(unknown.err(), Some(MeterError::NoChannel));
}
