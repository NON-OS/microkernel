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

use nonos_nox_meter::settle::{settle_data, settle_plan, RETRY_EVERY};
use nonos_nox_meter::MeterError;

use super::fixture::{sign, state, CHAIN, METER, NOW, SETTLEMENT};

#[test]
fn sign_carries_the_named_newest_vouchers_and_records_them() {
    let s = state();
    let (next, tx) = settle_plan(&s, &METER, &sign(4, &[1, 2]), &SETTLEMENT, NOW).unwrap();
    let want = [s.channels[0].signed(&METER).unwrap(), s.channels[1].signed(&METER).unwrap()];
    assert_eq!(tx.data, settle_data(&want));
    assert_eq!(
        (tx.chain_id, tx.to, tx.nonce, tx.gas, tx.value),
        (CHAIN, SETTLEMENT, 4, 250_000, 0)
    );
    assert_eq!((tx.max_fee_per_gas, tx.max_priority_fee_per_gas), (10, 2));
    assert_eq!((next.channels[0].carried, next.channels[1].carried), (300, 40));
    assert_eq!((next.channels[2].carried, next.last_settle), (0, Some((CHAIN, 4))));
}

#[test]
fn another_settle_needs_a_newer_voucher_the_same_nonce_or_a_day() {
    let (s, _) = settle_plan(&state(), &METER, &sign(4, &[1, 2]), &SETTLEMENT, NOW).unwrap();
    assert!(settle_plan(&s, &METER, &sign(4, &[1, 2]), &SETTLEMENT, NOW).is_ok(), "a fee bump");
    let stale = settle_plan(&s, &METER, &sign(3, &[1, 2]), &SETTLEMENT, NOW);
    assert_eq!(stale.err(), Some(MeterError::NothingOwed), "a nonce not past the last");
    // Nonce 4 went to another transaction: the same vouchers again, once a day.
    let (r, _) = settle_plan(&s, &METER, &sign(5, &[1, 2]), &SETTLEMENT, NOW).unwrap();
    assert_eq!((r.last_retry, r.last_settle), (NOW, Some((CHAIN, 5))));
    let soon = settle_plan(&r, &METER, &sign(6, &[1, 2]), &SETTLEMENT, NOW + RETRY_EVERY - 1);
    assert_eq!(soon.err(), Some(MeterError::NothingOwed), "a lying settle gets one a day");
    assert!(settle_plan(&r, &METER, &sign(6, &[1, 2]), &SETTLEMENT, NOW + RETRY_EVERY).is_ok());
    let mut newer = r.clone();
    newer.channels[1].cumulative = 41;
    let (n, _) = settle_plan(&newer, &METER, &sign(6, &[1, 2]), &SETTLEMENT, NOW + 1).unwrap();
    assert_eq!((n.channels[1].carried, n.last_retry), (41, NOW), "a newer voucher is no retry");
}

#[test]
fn nothing_unsigned_settled_or_unknown_is_carried() {
    let s = state();
    let refused = |ids: &[u8]| settle_plan(&s, &METER, &sign(4, ids), &SETTLEMENT, NOW).err();
    assert_eq!(refused(&[3]), Some(MeterError::NothingOwed), "unsigned");
    assert_eq!(refused(&[1, 9]), Some(MeterError::NoChannel), "unknown");
    let mut paid = s.clone();
    paid.channels[0].settled = 300;
    let r = settle_plan(&paid, &METER, &sign(4, &[1]), &SETTLEMENT, NOW).err();
    assert_eq!(r, Some(MeterError::NothingOwed), "already settled");
    let mut other = sign(4, &[1]);
    other.chain_id = 1;
    assert_eq!(
        settle_plan(&s, &METER, &other, &SETTLEMENT, NOW).err(),
        Some(MeterError::NoChannel)
    );
}
