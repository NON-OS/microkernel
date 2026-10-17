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

/* The nonce a publisher transaction is reviewed and signed at: the send
 * ledger's one rule both paths take, GAS's pending nonce never under one
 * past the highest this account's sends may still use; GAS alone is not
 * trusted, and another account's or network's sends are not counted. */

use nonos_settle_proto::{Gas, Quote};

use super::fixture::{at, quote};
use super::ledger::{bounds, entry, gas, send, ME};
use crate::intent::nonce::{after_gas, nonce_for, sign_at, NO_NONCE};
use crate::intent::{max_cost, IntentStep, NONCE_MOVED, NO_FUNDS};
use crate::sent::{Ledger, Sent};

#[test]
fn the_shared_nonce_is_gas_or_one_past_the_ledger_whichever_is_higher() {
    let s = send();
    assert_eq!(nonce_for(&Ledger::default(), &s, 7), Some(7), "an empty ledger: GAS");
    let mut l = Ledger::default();
    l.record(entry(ME, 7, 1));
    assert_eq!(nonce_for(&l, &s, 5), Some(8), "GAS lower than the ledger");
    assert_eq!(nonce_for(&l, &s, 7), Some(8), "GAS equal to the ledger's highest");
    assert_eq!(nonce_for(&l, &s, 8), Some(8), "GAS equal to one past it");
    assert_eq!(nonce_for(&l, &s, 12), Some(12), "GAS higher");
    let mut others = Ledger::default();
    others.record(entry([8; 20], 20, 2));
    others.record(Sent { chain: 5, ..entry(ME, 30, 3) });
    assert_eq!(nonce_for(&others, &s, 5), Some(5), "another account's, another network's");
    assert_eq!(nonce_for(&Ledger::default(), &s, u64::MAX), None, "EIP-2681");
}

#[test]
fn the_review_is_at_the_shared_nonce_and_sign_holds_it_there() {
    let (s, q) = (send(), quote(send().kind));
    let mut l = Ledger::default();
    l.record(entry(ME, 6, 1));
    let review = IntentStep::Review { quote: Quote { nonce: 7, ..q } };
    assert_eq!(after_gas(&l, &s, IntentStep::Funds(q), &gas(4)), review, "never GAS's 4");
    assert_eq!(after_gas(&Ledger::default(), &s, IntentStep::Funds(q), &gas(4)), review_at(q, 4));
    let poor = Gas { balance: max_cost(&q) - 1, ..gas(4) };
    assert_eq!(after_gas(&l, &s, IntentStep::Funds(q), &poor), IntentStep::Failed(NO_FUNDS));
    let none = after_gas(&Ledger::default(), &s, IntentStep::Funds(q), &gas(u64::MAX));
    assert_eq!(none, IntentStep::Failed(NO_NONCE));
    let t = sign_at(&l, &s, &review, 1, 4, &at, bounds).unwrap();
    assert_eq!(t.nonce, 7, "GAS lagging: the ledger's floor");
    let gas_alone = review_at(q, 4);
    assert_eq!(sign_at(&l, &s, &gas_alone, 1, 4, &at, bounds), Err(NONCE_MOVED));
}

fn review_at(q: Quote, nonce: u64) -> IntentStep {
    IntentStep::Review { quote: Quote { nonce, ..q } }
}
