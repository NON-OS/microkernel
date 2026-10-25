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

/* After Sign: what was signed is in the one send ledger before it goes,
 * and both paths see it there; each path's waiting sends hold the other;
 * a moved nonce is quoted again, read-only, and reviewed afresh, no press
 * standing, so nothing is signed until the user's next guarded Sign. */

use super::fixture::{at, quote};
use super::ledger::{entry, gas, send, ME};
use crate::intent::nonce::{after_gas, nonce_for};
use crate::intent::retry::{after_sign, Turn};
use crate::intent::{after_quote, max_cost, IntentStep, NONCE_MOVED, NOT_REVIEWED};
use crate::sent::{Fate, Ledger, Reading};

#[test]
fn a_signed_publisher_send_is_in_the_shared_ledger_before_it_goes() {
    let (s, q, mut l) = (send(), quote(send().kind), Ledger::default());
    let turn = after_sign(&mut l, &s, Ok((q, [3; 32])), 50);
    assert_eq!(turn, Turn { step: IntentStep::Broadcasting, pressed: false });
    let e = l.get(&[3; 32]).copied().expect("recorded");
    assert_eq!((e.chain, e.from, e.nonce, e.fate), (1, ME, q.nonce, Fate::Waiting));
    assert_eq!((e.eth_cost, e.token, e.at_ms), (max_cost(&q), None, 50));
    assert!(l.waiting_of(1, &ME, true) && !l.waiting_of(1, &ME, false));
    assert_eq!(nonce_for(&l, &s, q.nonce), Some(q.nonce + 1), "a payment's next is past it");
    assert_eq!(l.held(1, &ME, 0), (max_cost(&q), 0), "its fee is held back from a payment");
}

#[test]
fn each_paths_waiting_sends_hold_the_other_and_not_itself() {
    let mut l = Ledger::default();
    l.record(entry(ME, 4, 1));
    assert!(l.waiting_of(1, &ME, false), "a payment waiting holds the publisher");
    assert!(!l.waiting_of(1, &ME, true), "and does not hold another payment");
    l.record_publisher(entry(ME, 5, 2));
    assert!(l.waiting_of(1, &ME, true), "a publisher send waiting holds a payment");
    assert!(!l.waiting_of(5, &ME, true) && !l.waiting_of(1, &[8; 20], true));
    let block = Reading { receipt: Some((10, true)), head: 10, latest: 6, pending: 6 };
    l.judge(&[1; 32], block, 1);
    l.judge(&[2; 32], block, 1);
    assert!(!l.waiting_of(1, &ME, false) && !l.waiting_of(1, &ME, true), "in a block: free");
}

#[test]
fn a_moved_nonce_is_quoted_again_and_reviewed_with_no_press_standing() {
    let (s, q, mut l) = (send(), quote(send().kind), Ledger::default());
    l.record(entry(ME, 4, 1));
    let turn = after_sign(&mut l, &s, Err(NONCE_MOVED), 1);
    assert_eq!(turn, Turn { step: IntentStep::Quoting, pressed: false });
    assert_eq!(l.list.len(), 1, "nothing signed, nothing recorded");
    let funds = after_quote(&s, &q, &at);
    assert_eq!(funds, IntentStep::Funds(q), "the re-quote: INTENT, read-only");
    let review = after_gas(&l, &s, funds, &gas(4));
    let IntentStep::Review { quote: again } = review else { panic!("{review:?}") };
    assert_eq!(again.nonce, 5, "at the shared nonce, past the payment");
    let other = after_sign(&mut l, &s, Err(NOT_REVIEWED), 1);
    assert_eq!(other, Turn { step: IntentStep::Failed(NOT_REVIEWED), pressed: false });
}
