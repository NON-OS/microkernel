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

/* A publisher transaction's last pure steps: the balance against the most
 * the quote can cost, and no signature once the chain's pending nonce is
 * not the quoted one (Q8). */

use super::fixture::quote;
use crate::intent::{after_balance, max_cost, sign, IntentStep};
use crate::intent::{NONCE_MOVED, NOT_REVIEWED, NO_FUNDS};

#[test]
fn the_balance_covers_the_most_it_can_cost() {
    let q = quote(2);
    let most = max_cost(&q);
    assert_eq!(after_balance(IntentStep::Funds(q), most - 1), IntentStep::Failed(NO_FUNDS));
    assert_eq!(after_balance(IntentStep::Funds(q), most), IntentStep::Review { quote: q });
    for other in [IntentStep::Quoting, IntentStep::Landed, IntentStep::Sent([1; 32])] {
        assert_eq!(after_balance(other, u128::MAX), other, "only Funds moves on a balance");
    }
}

#[test]
fn no_signature_once_the_nonce_moved() {
    let q = quote(4);
    let review = IntentStep::Review { quote: q };
    assert_eq!(sign(&review, q.nonce), Ok(q));
    for pending in [q.nonce + 1, q.nonce - 1, 0, u64::MAX] {
        assert_eq!(sign(&review, pending), Err(NONCE_MOVED), "pending {pending}");
    }
    for s in [IntentStep::Funds(q), IntentStep::Quoting, IntentStep::Broadcasting] {
        assert_eq!(sign(&s, q.nonce), Err(NOT_REVIEWED), "{s:?}");
    }
}
