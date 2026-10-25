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

use nonos_nox_config::{address, Contract, SEPOLIA};
use nonos_settle_proto::Quote;

use crate::intent::{fee_rows, max_cost, quote_refusal, Publisher, PUBLISHER_GAS_MAX};

fn nox(v: u128) -> String {
    format!("{v}")
}

fn good() -> (Publisher, Quote) {
    let p = Publisher::Approve { spender: Contract::PublisherBond, amount: 5 };
    let to = address(SEPOLIA, Contract::NoxToken).unwrap();
    let q = Quote {
        kind: 1,
        to,
        nonce: 4,
        gas_limit: 60_000,
        max_fee: 2_000_000_000,
        max_priority: 100_000_000,
    };
    (p, q)
}

#[test]
fn a_quote_is_held_to_the_intent_and_the_caps() {
    let (p, q) = good();
    assert_eq!(quote_refusal(&p, SEPOLIA, &q), None);
    let bad = |f: fn(&mut Quote)| {
        let mut b = q;
        f(&mut b);
        quote_refusal(&p, SEPOLIA, &b).is_some()
    };
    assert!(bad(|q| q.kind = 2));
    assert!(bad(|q| q.to[0] ^= 1));
    assert!(bad(|q| q.gas_limit = 0));
    assert!(bad(|q| q.gas_limit = PUBLISHER_GAS_MAX + 1));
    assert!(!bad(|q| q.gas_limit = PUBLISHER_GAS_MAX));
    assert!(bad(|q| q.max_fee = 0));
    assert!(bad(|q| q.max_fee = 1_000_000_000_001));
    assert!(bad(|q| q.max_priority = q.max_fee + 1));
    assert!(bad(|q| {
        q.max_fee = 50_000_000_000;
        q.max_priority = 10_000_000_001
    }));
    let bond = Publisher::Bond { publisher_key: [1; 32], amount: 1 };
    assert!(quote_refusal(&bond, SEPOLIA, &Quote { kind: 2, ..q }).is_some(), "no Bond deployed");
}

#[test]
fn the_fee_row_is_the_most_it_can_cost() {
    let (_, q) = good();
    assert_eq!(max_cost(&q), 60_000 * 2_000_000_000);
    assert_eq!(fee_rows(&q, nox)[0].1, "at most 120000000000000 ETH");
    assert_eq!(max_cost(&Quote { gas_limit: u64::MAX, max_fee: u128::MAX, ..q }), u128::MAX);
}
