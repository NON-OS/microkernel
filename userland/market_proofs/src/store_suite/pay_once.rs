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
//! PAY of one install unit as capsule_payment's PAY decides it, `priced`
//! then `landing` and its second pass, beside FUNDED and the store's Enter.

use nonos_nox_config::{address, Contract, SEPOLIA};
use nonos_payment_proto::PayRequest;

use super::held_fixture::{sold, KEY, NOW};
use super::meter_fixture::{record, reply};
use super::meter_state::{PAYEE, UNIT};
use crate::meter::pay::{landing, priced};
use crate::meter::state::MeterState;
use crate::store::channel_held::{answer, route, Route};
use crate::store::price::VOUCHER;

/// PAY of one unit of the sold listing at 3, as the handler decides it.
fn pay(s: &MeterState) -> bool {
    let (listing_id, release_hash, call_id) = (KEY, [2; 32], [4; 32]);
    let (qty, unit_price, chain_id) = (1, 3, SEPOLIA);
    let (payee, unit, listing, release) = (PAYEE, UNIT, &b"linux.qwen-small"[..], &b""[..]);
    let r = PayRequest {
        listing_id,
        release_hash,
        payee,
        unit,
        qty,
        unit_price,
        call_id,
        chain_id,
        listing,
        release,
    };
    let nox = address(SEPOLIA, Contract::NoxToken).expect("NOX on Sepolia");
    let Ok(amount) = priced(&r, &record(3), &nox) else { return false };
    landing(s, &r, amount, NOW).is_ok()
        && s.charged(&payee, &listing_id, chain_id, amount, [6; 32], NOW).is_ok()
}

/// Each case: PAY, FUNDED and the store's Enter all say `want`.
pub fn same(cases: &[(&str, MeterState, bool)]) {
    for (why, s, want) in cases {
        assert_eq!(pay(s), *want, "PAY: {why}");
        let to = route(&sold(VOUCHER), answer(Ok(reply(s, &record(3)))));
        assert_eq!(to == Route::Install, *want, "FUNDED: {why}");
    }
}
