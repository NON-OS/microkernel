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

/*
 * A publisher transaction's moves on settle's answers: the quote held to
 * the calldata's own decode and the wallet's bounds (quote.rs), then the
 * account's balance against the most the quote can cost.
 */

use nonos_settle_proto::{Quote, KINDS};

use super::decode::decode;
use super::quote::{held, max_cost};
use super::review_bond::At;
use super::send::PublisherSend;
use super::step::{IntentStep, NOT_OURS, NO_FUNDS};

/// `at` names the contracts' addresses on the send's chain.
pub fn after_quote(send: &PublisherSend, q: &Quote, at: At) -> IntentStep {
    let Some(p) = decode(send.kind, &send.calldata, at) else {
        return IntentStep::Failed(NOT_OURS);
    };
    let to = KINDS.iter().find(|k| k.id == send.kind).and_then(|k| at(k.contract));
    match held(&p, to, q) {
        Some(why) => IntentStep::Failed(why),
        None => IntentStep::Funds(*q),
    }
}

pub fn after_balance(step: IntentStep, balance: u128) -> IntentStep {
    match step {
        IntentStep::Funds(q) if balance < max_cost(&q) => IntentStep::Failed(NO_FUNDS),
        IntentStep::Funds(q) => IntentStep::Review { quote: q },
        other => other,
    }
}
