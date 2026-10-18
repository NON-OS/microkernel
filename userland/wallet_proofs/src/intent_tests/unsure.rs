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

/* A broadcast settle did not confirm is read on chain before anything:
 * its nonce used (GAS's pending past it), it is followed as sent and never
 * sent again; unused, the same transaction at the same nonce is offered
 * for Sign, as a review with no press standing and the guard afresh, never
 * broadcast. No restart case: the send ledger is not kept across one. */

use super::super::order_tests::press_grey::T;
use super::fixture::{at, quote};
use super::ledger::{bounds, send, ME};
use crate::intent::nonce::{nonce_for, sign_at};
use crate::intent::retry::{after_sign, Turn};
use crate::intent::unsure::{after_broadcast, after_unsure, NOT_SENT, UNFOLLOWED};
use crate::intent::IntentStep;
use crate::sent::{Fate, Ledger};
use crate::wallet::arm::look::{pressed, Arm};
use crate::wallet::arm::moves::Shown;

fn unsure(l: &mut Ledger) -> IntentStep {
    let (s, q) = (send(), quote(send().kind));
    after_sign(l, &s, Ok((q, [3; 32])), 1);
    after_broadcast(None, Some((q, [3; 32])))
}

#[test]
fn a_broadcast_answer_is_sent_and_none_is_unsure() {
    let q = quote(send().kind);
    let ours = IntentStep::Sent([3; 32]);
    assert_eq!(after_broadcast(Some([4; 32]), Some((q, [3; 32]))), ours, "the hash signed");
    let step = after_broadcast(None, Some((q, [3; 32])));
    assert_eq!(step, IntentStep::Unsure { quote: q, hash: [3; 32] });
    assert_eq!(after_broadcast(None, None), IntentStep::Failed(NOT_SENT));
    assert_eq!(after_broadcast(Some([4; 32]), None), IntentStep::Failed(UNFOLLOWED));
}

#[test]
fn its_nonce_used_it_is_followed_as_sent_and_never_sent_again() {
    let mut l = Ledger::default();
    let step = unsure(&mut l);
    let n = quote(send().kind).nonce;
    let turn = after_unsure(&mut l, step, n + 1);
    assert_eq!(turn, Turn { step: IntentStep::Sent([3; 32]), pressed: false });
    assert_eq!(l.get(&[3; 32]).map(|e| e.fate), Some(Fate::Waiting), "still followed");
    assert_eq!(nonce_for(&l, &send(), n), Some(n + 1), "its nonce stays taken");
}

#[test]
fn its_nonce_unused_the_same_send_is_offered_again_for_sign_only() {
    let (mut l, q) = (Ledger::default(), quote(send().kind));
    let step = unsure(&mut l);
    let turn = after_unsure(&mut l, step, q.nonce);
    assert_eq!(turn, Turn { step: IntentStep::Review { quote: q }, pressed: false });
    assert!(!l.waiting_of(1, &ME, true), "known unsent: a payment is free again");
    let t = sign_at(&l, &send(), &turn.step, 1, q.nonce, &at, bounds).unwrap();
    assert_eq!(t.nonce, q.nonce, "the same nonce, once Sign is pressed again");
    let shown = Some(Shown::PublisherReview);
    assert!(!pressed(Arm::default(), shown, true, T), "the guard afresh: not armed yet");
}
