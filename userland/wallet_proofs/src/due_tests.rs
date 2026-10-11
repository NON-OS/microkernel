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

//! The due list (v1.4.56): filled by a sent refund, a sent dispute and an
//! owed refund and by nothing else; only its addresses are read, each at
//! most once per REREAD_MS; an address whose money moved on leaves it.

use nonos_payment_proto::{DeliveredMade, OrderStatus};

use crate::order::send_step::SendStep;
use crate::shield::due::{Due, Why, MAX_DUE, REREAD_MS};
use crate::shield::due_why::due_why;
use crate::shield::landed_held::{both, Held};

fn n(nox: u128) -> Option<Held> {
    both(Some(nox), Some(0))
}

fn status(state: u8) -> OrderStatus {
    OrderStatus { state, funded: 9, pending: 0, plain: false }
}

#[test]
fn only_a_sent_refund_a_sent_dispute_or_an_owed_refund_is_due() {
    let sent = SendStep::Sent([1; 32]);
    let s = |n| Some(status(n));
    assert_eq!(due_why(Some((11, &sent)), None, None, 0, 0), Some(Why::RefundSent));
    assert_eq!(due_why(Some((10, &SendStep::Done)), None, None, 0, 0), Some(Why::DisputeSent));
    assert_eq!(due_why(None, s(3).as_ref(), None, 9, 0), Some(Why::RefundOwed), "disputed");
    let late = due_why(None, s(1).as_ref(), None, 9, 9);
    assert_eq!(late, Some(Why::RefundOwed), "past the deadline");
    assert_eq!(due_why(None, s(1).as_ref(), None, 9, 8), None, "funded, in time");
    let bad = DeliveredMade::BadChunk { index: 0, calldata: vec![] };
    assert_eq!(due_why(None, s(2).as_ref(), Some(&bad), 9, 0), None, "a dispute offered");
    assert_eq!(due_why(Some((11, &SendStep::Reading)), s(3).as_ref(), None, 9, 0), None);
    assert_eq!(due_why(None, None, None, 0, u64::MAX), None, "no status");
}

#[test]
fn only_due_addresses_are_read_and_not_again_within_ten_minutes() {
    let mut d = Due::default();
    assert_eq!(d.next(0), None, "nothing due, nothing read");
    d.add([1; 20], Why::RefundOwed, 0);
    d.add([1; 20], Why::RefundSent, 5);
    assert_eq!(d.entries.len(), 1, "once per address");
    assert_eq!(d.next(0), Some([1; 20]));
    d.read([1; 20], n(0), 100);
    assert_eq!(d.next(100 + REREAD_MS - 1), None, "not again within REREAD_MS");
    assert_eq!(d.next(100 + REREAD_MS), Some([1; 20]));
    d.read([2; 20], n(5), 100);
    assert_eq!(d.entries.len(), 1, "a read of an address not due changes nothing");
    d.read([1; 20], n(7), 200);
    assert_eq!(d.entries[0].held, n(7));
    d.read([1; 20], None, 300);
    assert_eq!(d.entries[0].held, n(7), "no answer keeps what was seen");
    d.read([1; 20], n(0), 400);
    assert!(d.entries.is_empty(), "moved on: no longer due");
    (0..=MAX_DUE as u8).for_each(|i| d.add([i; 20], Why::RefundOwed, 0));
    assert_eq!((d.entries.len(), d.entries[0].address), (MAX_DUE, [1; 20]), "bounded");
}
