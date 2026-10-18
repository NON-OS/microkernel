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

/* The hash shown first is the one the ledger follows, computed from the
 * bytes signed, never settle's answer; an answer naming another is said
 * apart, as a warning. A dropped send is offered again only as a review,
 * no press standing, once its nonce reads unused: never sent by itself. */

use super::followed::{sent, words, NODE, OURS};
use crate::hex_id::id;
use crate::intent::followed::{after_reread, dropped, echo, echo_said, may_land};
use crate::intent::retry::Turn;
use crate::intent::IntentStep;
use crate::sent::Fate;

#[test]
fn the_hash_shown_is_the_ledgers_own_when_settle_answers_another() {
    let (l, step, q) = sent(None);
    assert_eq!(step, IntentStep::Sent(OURS), "followed under the hash signed, not NODE");
    assert_eq!(l.get(&OURS).map(|e| e.fate), Some(Fate::Waiting));
    assert!(l.get(&NODE).is_none(), "settle's answer is in no ledger entry");
    let w = words(&l, &step);
    assert!(w.starts_with(&format!("Sent {}", id(&OURS))) && !w.contains(&id(&NODE)), "{w}");
    assert_eq!(echo(Some(NODE), Some((q, OURS))), Some(NODE), "kept apart, to be said");
    assert_eq!(echo(Some(OURS), Some((q, OURS))), None, "the same hash: nothing more said");
    assert_eq!(echo(None, Some((q, OURS))), None);
}

#[test]
fn the_nodes_answer_is_labelled_on_its_own_line_as_a_warning() {
    let said = echo_said(Some(NODE)).expect("said when it differs");
    assert!(said.starts_with("Warning: the node answered "), "{said}");
    assert!(said.contains(&id(&NODE)) && said.contains("never the node's"), "{said}");
    assert_eq!(echo_said(None), None);
}

#[test]
fn a_dropped_send_never_goes_by_itself_and_only_its_review_returns() {
    let (mut l, step, q) = sent(Some((None, 10, 0)));
    assert!(dropped(&l, &step) && !may_land(&l, &step));
    let signed = Some((q, OURS));
    for pending in [0, q.nonce, q.nonce + 1, q.nonce + 9] {
        let turn = after_reread(&mut l.clone(), step, signed, pending);
        assert!(!turn.pressed && turn.step != IntentStep::Broadcasting, "{turn:?}");
    }
    let used = after_reread(&mut l.clone(), step, signed, q.nonce + 1);
    assert_eq!(used, Turn { step, pressed: false }, "its nonce used: Cancel only");
    let unused = after_reread(&mut l, step, signed, q.nonce);
    assert_eq!(unused, Turn { step: IntentStep::Review { quote: q }, pressed: false });
    assert_eq!(l.get(&OURS).map(|e| e.fate), Some(Fate::Dropped), "nothing sent again");
}

#[test]
fn a_waiting_or_replaced_send_is_never_offered_again() {
    for read in [None, Some((None, 10, 1))] {
        let (mut l, step, q) = sent(read);
        for pending in [0, q.nonce, q.nonce + 1] {
            let turn = after_reread(&mut l, step, Some((q, OURS)), pending);
            assert_eq!(turn, Turn { step, pressed: false }, "{read:?} at {pending}");
        }
    }
}
