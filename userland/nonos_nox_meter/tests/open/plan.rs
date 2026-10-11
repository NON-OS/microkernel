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

use super::fixture::{gas, start, NOX, SETTLEMENT};
use nonos_nox_meter::open::{open_plan, sign_tx_request};
use nonos_nox_meter::MeterError;
use nonos_payment_proto::OpenStart;
use nonos_settle_proto::Gas;

#[test]
fn approve_then_open_at_the_next_two_nonces() {
    let p = open_plan(&start(), &NOX, &SETTLEMENT, &gas()).unwrap();
    assert_eq!(
        (p.approve.to, p.approve.nonce, p.approve.gas, p.approve.value),
        (NOX, 7, 60_000, 0)
    );
    assert_eq!((p.open.to, p.open.nonce, p.open.gas, p.open.value), (SETTLEMENT, 8, 200_000, 0));
    assert_eq!(
        (p.open.chain_id, p.open.max_fee_per_gas, p.open.max_priority_fee_per_gas),
        (11_155_111, 10, 2)
    );
    let r = sign_tx_request(42, 3, &p.approve);
    assert_eq!(r.len(), 200 + 68);
    assert_eq!(
        (&r[..4], &r[4..8], &r[8..16]),
        (&42u32.to_le_bytes()[..], &3u32.to_le_bytes()[..], &11_155_111u64.to_le_bytes()[..])
    );
    assert_eq!((&r[16..36], r[67], r[195]), (&NOX[..], 7, 0));
    assert_eq!(&r[196..200], &68u32.to_le_bytes());
}

#[test]
fn zero_amounts_short_gas_and_fees_past_the_caps_are_refused() {
    let zero = OpenStart { amount: 0, ..start() };
    assert_eq!(open_plan(&zero, &NOX, &SETTLEMENT, &gas()).err(), Some(MeterError::Mismatch));
    let short = Gas { balance: 260_000 * 10 - 1, ..gas() };
    assert_eq!(open_plan(&start(), &NOX, &SETTLEMENT, &short).err(), Some(MeterError::NoGas));
    let (gwei, rich) = (1_000_000_000u128, u128::MAX);
    for (fee, tip) in [(1_000 * gwei + 1, 1), (11 * gwei, 10 * gwei + 1), (10, 11)] {
        let g = Gas { balance: rich, max_fee_per_gas: fee, max_priority_fee_per_gas: tip, ..gas() };
        assert_eq!(
            open_plan(&start(), &NOX, &SETTLEMENT, &g).err(),
            Some(MeterError::NoGas),
            "{fee} {tip}"
        );
    }
    let at_caps = Gas {
        balance: rich,
        max_fee_per_gas: 1_000 * gwei,
        max_priority_fee_per_gas: 10 * gwei,
        ..gas()
    };
    assert!(open_plan(&start(), &NOX, &SETTLEMENT, &at_caps).is_ok());
}
