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
//! FUNDED's record half: the price is the listing's own, and a record PAY
//! would refuse answers not funded, however much a channel holds.

use nonos_marketplace_abi::{commerce_bytes, Commerce, SettleMode};
use nonos_nox_meter::pay::{funded, install, Offer};
use nonos_nox_meter::state::MeterState;
use nonos_payment_proto::PayRequest;

use super::fixture::{request, NOX};
use super::funded_fixture::{channel, record, sold, state, NOW};

fn held() -> MeterState {
    state(&[channel()])
}

#[test]
fn the_price_is_the_records_unit_price() {
    let one = PayRequest { qty: 1, call_id: [0; 32], listing: b"", release: b"", ..request() };
    assert_eq!(install(&record(), 1), Some(one), "one unit, the record's payee and unit");
    let dear = Offer { block: commerce_bytes(&Commerce { unit_price: 15, ..sold() }), ..record() };
    assert_eq!(install(&dear, 1).map(|r| r.unit_price), Some(15));
    assert_eq!(install(&Offer { block: vec![1, 2, 3], ..record() }, 1), None);
    assert!(!funded(&held(), &dear, 1, &NOX, NOW), "14 left, the record says 15");
}

#[test]
fn a_record_pay_would_refuse_is_not_funded() {
    let block = |c: Commerce| Offer { block: commerce_bytes(&c), ..record() };
    for bad in [
        Offer { trusted: false, ..record() },
        Offer { chain_id: 2, ..record() },
        Offer { token: vec![0; 20], ..record() },
        Offer { block: vec![1, 2, 3], ..record() },
        Offer { listing_id: [8; 32], ..record() },
        block(Commerce { settle_mode: SettleMode::Escrow, ..sold() }),
        block(Commerce { settle_mode: SettleMode::Free, ..sold() }),
        block(Commerce { unit_price: 0, min_charge: 0, ..sold() }),
        block(Commerce { min_charge: 8, ..sold() }),
    ] {
        assert!(!funded(&held(), &bad, 1, &NOX, NOW), "{bad:?}");
    }
    assert!(!funded(&held(), &record(), 2, &NOX, NOW), "asked on another chain");
    assert!(!funded(&held(), &record(), 1, &[0x0b; 20], NOW), "not this chain's NOX");
}
