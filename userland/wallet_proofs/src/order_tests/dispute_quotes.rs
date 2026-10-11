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

//! settle's quote for a dispute held to its bounds, and the gas withdraw's
//! size, the smallest standard ETH amount at or above the cost (v1.4.51).

use nonos_nox_config::{DISPUTE_GAS, MAX_FEE_PER_GAS};
use nonos_settle_proto::Quote;

use super::dispute_fixture::{quote, ESCROW};
use crate::order::dispute_quote::{gas_size, quote_refusal};

const CENTI: u128 = 10_000_000_000_000_000;

#[test]
fn a_quote_out_of_bounds_is_refused() {
    let q = quote();
    assert_eq!(quote_refusal(&q, Some(ESCROW)), None);
    let refused = |q: Quote, escrow| quote_refusal(&q, escrow).is_some();
    assert!(refused(Quote { kind: 11, ..q }, Some(ESCROW)), "a refund's quote");
    assert!(refused(q, Some([1; 20])), "another contract");
    assert!(refused(q, None), "no Escrow in the config");
    assert!(refused(Quote { gas_limit: DISPUTE_GAS + 1, ..q }, Some(ESCROW)));
    assert!(refused(Quote { max_fee: MAX_FEE_PER_GAS + 1, ..q }, Some(ESCROW)));
    assert!(refused(Quote { max_priority: q.max_fee + 1, ..q }, Some(ESCROW)));
}

#[test]
fn the_gas_withdraw_is_the_smallest_standard_size_that_covers_it() {
    let at = |gas_limit: u64, max_fee: u128| gas_size(&Quote { gas_limit, max_fee, ..quote() });
    assert_eq!(at(300_000, 20_000_000_000), Some(CENTI), "0.006 ETH: 0.01");
    assert_eq!(at(1_000_000, 11_000_000_000), Some(2 * CENTI), "0.011 ETH: 0.02");
    assert_eq!(at(1, 10 * CENTI), Some(10 * CENTI), "exactly 0.1 ETH");
    assert_eq!(at(400_000, MAX_FEE_PER_GAS), Some(50 * CENTI), "0.4 ETH: 0.5");
    assert_eq!(at(1, 1_000 * CENTI), Some(1_000 * CENTI), "10 ETH, the largest");
    assert_eq!(at(1, 1_000 * CENTI + 1), None, "past 10 ETH");
}
