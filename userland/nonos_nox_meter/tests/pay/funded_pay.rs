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

//! FUNDED is one exactly when PAY of one install unit would charge: the
//! request the store's install sends, run through what capsule_payment's
//! PAY runs, `priced` then pay_charge.rs's `landing` and its second pass.

use nonos_nox_meter::channel::{Channel, CHARGE_MARGIN};
use nonos_nox_meter::pay::{funded, landing, priced};
use nonos_nox_meter::state::MeterState;
use nonos_payment_proto::PayRequest;

use super::fixture::{request, NOX};
use super::funded_fixture::{channel, record, state, NOW};

/// PAY of one unit, as the handler decides it.
fn pay(s: &MeterState) -> bool {
    let (r, o) = (PayRequest { qty: 1, ..request() }, record());
    let Ok(amount) = priced(&r, &o, &NOX) else { return false };
    landing(s, &r, amount, NOW).is_ok()
        && s.charged(&r.payee, &r.listing_id, r.chain_id, amount, [6; 32], NOW).is_ok()
}

/// Each case answers `want` from PAY and from FUNDED alike.
pub fn same(cases: &[(&str, MeterState, bool)]) {
    for (why, s, want) in cases {
        assert_eq!(pay(s), *want, "PAY: {why}");
        assert_eq!(funded(s, &record(), 1, &NOX, NOW), *want, "FUNDED: {why}");
    }
}

#[test]
fn funded_is_one_exactly_when_pay_of_one_unit_charges() {
    let on = |c: Channel| state(&[c]);
    let frozen_long = Channel { id: [6; 32], frozen: true, expiry: NOW * 2, ..channel() };
    same(&[
        ("funded", state(&[channel()]), true),
        ("one install left", on(Channel { cumulative: 13, ..channel() }), true),
        ("prefers the funded", state(&[frozen_long, channel()]), true),
        ("payee mismatch", on(Channel { payee: [0xcc; 20], ..channel() }), false),
        ("unit mismatch", on(Channel { unit: [4; 32], ..channel() }), false),
        ("no row", state(&[]), false),
        ("frozen", on(Channel { frozen: true, ..channel() }), false),
        ("within the margin", on(Channel { expiry: NOW + CHARGE_MARGIN, ..channel() }), false),
        ("past the margin", on(Channel { expiry: NOW + CHARGE_MARGIN + 1, ..channel() }), true),
        ("underfunded", on(Channel { cumulative: 14, ..channel() }), false),
        ("settled counts", on(Channel { settled: 14, cumulative: 0, ..channel() }), false),
        ("overflow", on(Channel { cumulative: u128::MAX, deposit: u128::MAX, ..channel() }), false),
        ("seq wraps", on(Channel { seq: u64::MAX, ..channel() }), false),
        ("another listing", on(Channel { listing_id: [8; 32], ..channel() }), false),
        ("another chain", on(Channel { chain_id: 2, ..channel() }), false),
    ]);
}
