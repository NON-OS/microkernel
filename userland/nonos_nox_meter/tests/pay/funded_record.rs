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
use nonos_nox_meter::pay::{committed, funded, Offer};

use super::fixture::NOX;
use super::funded_fixture::{record, row, sold, NOW};

#[test]
fn the_price_is_the_records_unit_price() {
    assert_eq!(committed(&record(), 1, &NOX), Some(7));
    let dear = Offer { block: commerce_bytes(&Commerce { unit_price: 15, ..sold() }), ..record() };
    assert_eq!(committed(&dear, 1, &NOX), Some(15));
    assert!(!funded([row()], &dear, 1, &NOX, NOW), "14 left, the record says 15");
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
        assert!(!funded([row()], &bad, 1, &NOX, NOW), "{bad:?}");
    }
    assert!(!funded([row()], &record(), 2, &NOX, NOW), "asked on another chain");
    assert!(!funded([row()], &record(), 1, &[0x0b; 20], NOW), "not this chain's NOX");
}
